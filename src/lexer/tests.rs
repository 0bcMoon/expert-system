// Run all tests: cargo test
// Run lexer tests only: cargo test lexer
// Add a test by copying an example, renaming it, and changing its input and checks.
// Later, the parser and engine can each have their own tests.rs in the same way.

// Import items from the parent lexer module, including private items.
use super::*;

// #[test] tells Rust to run this function as a test.
#[test]
fn tokenizes_simple_and_expression() {
    // These tests tokenize a line directly, so this filename is never opened.
    let mut lexer = Lexer::new(String::from("unused.txt"));

    // expect fails the test with this message if tokenization returns an error.
    lexer
        .tokenize_line(&String::from("A+B"))
        .expect("A+B should tokenize successfully");

    // assert_eq! checks that two values are equal.
    assert_eq!(lexer.tokens.len(), 3);
    // matches! checks an enum variant; assert! requires the result to be true.
    assert!(matches!(lexer.tokens[0].kind, TokenKind::Identifier('A')));
    assert!(matches!(lexer.tokens[1].kind, TokenKind::And));
    assert!(matches!(lexer.tokens[2].kind, TokenKind::Identifier('B')));
}

#[test]
fn tokenizes_simple_and_expression1() {
    // These tests tokenize a line directly, so this filename is never opened.
    let mut lexer = Lexer::new(String::from("unused.txt"));

    // expect fails the test with this message if tokenization returns an error.
    lexer
        .tokenize_line(&String::from("A+B => A"))
        .expect("A+B => A should tokenize successfully");

    // assert_eq! checks that two values are equal.
    assert_eq!(lexer.tokens.len(), 5);
    // matches! checks an enum variant; assert! requires the result to be true.
    assert!(matches!(lexer.tokens[0].kind, TokenKind::Identifier('A')));
    assert!(matches!(lexer.tokens[1].kind, TokenKind::And));
    assert!(matches!(lexer.tokens[2].kind, TokenKind::Identifier('B')));
    assert!(matches!(lexer.tokens[3].kind, TokenKind::Implies));
    assert!(matches!(lexer.tokens[4].kind, TokenKind::Identifier('A')));
}

#[test]
fn rejects_unknown_character() {
    let mut lexer = Lexer::new(String::from("unused.txt"));

    let result = lexer.tokenize_line(&String::from("@"));

    // Internal index 0 becomes position 1 in the error message.
    assert!(matches!(
        result,
        Err(LexerError::UnexpectedCharacter {
            character: '@',
            line: 1,
            column: 1,
        })
    ));
}

// Small helpers keep each test focused on its input and expected output.
fn lex_line(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(String::from("unused.txt"));
    lexer.tokenize_line(source)?;
    Ok(lexer.tokens)
}

fn assert_kinds(tokens: &[Token], expected: &[TokenKind]) {
    assert_eq!(tokens.len(), expected.len(), "tokens: {tokens:?}");
    for (token, kind) in tokens.iter().zip(expected) {
        assert_eq!(
            std::mem::discriminant(&token.kind),
            std::mem::discriminant(kind),
            "expected {kind:?}, got {token:?}"
        );
        if let (TokenKind::Identifier(actual), TokenKind::Identifier(expected)) =
            (&token.kind, kind)
        {
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn tokenizes_uppercase_and_lowercase_identifiers() {
    let source = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let tokens = lex_line(source).unwrap();
    let expected: Vec<_> = source.chars().map(TokenKind::Identifier).collect();
    assert_kinds(&tokens, &expected);
}

#[test]
fn tokenizes_all_single_character_operators() {
    let tokens = lex_line("+|^!?()").unwrap();
    assert_kinds(
        &tokens,
        &[
            TokenKind::And,
            TokenKind::Or,
            TokenKind::Xor,
            TokenKind::Not,
            TokenKind::Query,
            TokenKind::Oparenth,
            TokenKind::Cparenth,
        ],
    );
    for (index, token) in tokens.iter().enumerate() {
        assert_eq!(token.pos, index);
    }
}

#[test]
fn ignores_empty_lines_whitespace_and_comments() {
    for source in ["", " \t\r", "# comment", "  # @ <=> ignored"] {
        assert!(lex_line(source).unwrap().is_empty(), "input: {source:?}");
    }
}

#[test]
fn ignores_inline_comments() {
    assert_kinds(
        &lex_line("A+B# @ invalid characters are comments").unwrap(),
        &[
            TokenKind::Identifier('A'),
            TokenKind::And,
            TokenKind::Identifier('B'),
        ],
    );
}

#[test]
fn preserves_positions_across_whitespace() {
    let tokens = lex_line(" A\t+ B ").unwrap();
    assert_kinds(
        &tokens,
        &[
            TokenKind::Identifier('A'),
            TokenKind::And,
            TokenKind::Identifier('B'),
        ],
    );
    assert_eq!(
        tokens.iter().map(|token| token.pos).collect::<Vec<_>>(),
        vec![1, 3, 5]
    );
}

#[test]
fn tokenizes_implication_with_or_without_spaces() {
    for source in ["A=>B", "A => B", "A+B => C"] {
        let tokens = lex_line(source).unwrap();
        let implication = tokens
            .iter()
            .find(|t| matches!(t.kind, TokenKind::Implies))
            .unwrap();
        // A multi-character token should point to its first character.
        assert_eq!(
            implication.pos,
            source.find('=').unwrap(),
            "input: {source}"
        );
    }
}

#[test]
fn tokenizes_equivalence() {
    for source in ["A<=>B", "A <=> B"] {
        let tokens = lex_line(source).unwrap();
        assert_kinds(
            &tokens,
            &[
                TokenKind::Identifier('A'),
                TokenKind::IfAndOnlyIf,
                TokenKind::Identifier('B'),
            ],
        );
        assert_eq!(tokens[1].pos, source.find('<').unwrap());
    }
}

#[test]
fn tokenizes_initial_facts_and_queries() {
    assert_kinds(
        &lex_line("=ABG").unwrap(),
        &[
            TokenKind::Equal,
            TokenKind::Identifier('A'),
            TokenKind::Identifier('B'),
            TokenKind::Identifier('G'),
        ],
    );
    assert_kinds(
        &lex_line("?GVX").unwrap(),
        &[
            TokenKind::Query,
            TokenKind::Identifier('G'),
            TokenKind::Identifier('V'),
            TokenKind::Identifier('X'),
        ],
    );
}

#[test]
fn accepts_empty_initial_facts() {
    // The sample language allows '=' when no facts are initially true.

    let i = &lex_line("=").unwrap();
    assert_kinds(i, &[TokenKind::Equal]);
}

#[test]
fn accepts_spaces_and_comments_after_initial_facts_marker() {
    for source in ["= A", "=\tA", "= # no initial facts"] {
        let tokens = lex_line(source).unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Equal));
        if source.contains('A') {
            assert_kinds(&tokens, &[TokenKind::Equal, TokenKind::Identifier('A')]);
        } else {
            assert_eq!(tokens.len(), 1);
        }
    }
}

#[test]
fn tokenizes_nested_parentheses_and_negation() {
    assert_kinds(
        &lex_line("!(A|(!B^C))").unwrap(),
        &[
            TokenKind::Not,
            TokenKind::Oparenth,
            TokenKind::Identifier('A'),
            TokenKind::Or,
            TokenKind::Oparenth,
            TokenKind::Not,
            TokenKind::Identifier('B'),
            TokenKind::Xor,
            TokenKind::Identifier('C'),
            TokenKind::Cparenth,
            TokenKind::Cparenth,
        ],
    );
}

#[test]
fn leaves_expression_validation_to_parser() {
    // These tokens are recognizable even though this is not a valid expression.
    assert_kinds(
        &lex_line("A++B)").unwrap(),
        &[
            TokenKind::Identifier('A'),
            TokenKind::And,
            TokenKind::And,
            TokenKind::Identifier('B'),
            TokenKind::Cparenth,
        ],
    );
}

#[test]
fn rejects_unsupported_characters_at_correct_positions() {
    for character in ['0', '@', '&', '*', '>', 'é', '🦀'] {
        let result = lex_line(&format!("A {character}"));
        assert!(
            matches!(result, Err(LexerError::UnexpectedCharacter {
            character: actual, line: 1, column: 3,
        }) if actual == character),
            "input character: {character}, result: {result:?}"
        );
    }
}

#[test]
fn rejects_malformed_equivalence() {
    for source in ["<AB", "<>A", "<<>", "A <== B"] {
        assert!(lex_line(source).is_err(), "should reject: {source}");
    }
}

#[test]
fn reports_incomplete_equivalence_at_end_of_input() {
    for source in ["<", "<=", "A <", "A <="] {
        let result = lex_line(source);
        assert!(
            matches!(result, Err(LexerError::UnexpectedEof {
            line: 1, column,
        }) if column == source.chars().count() + 1),
            "input: {source}, result: {result:?}"
        );
    }
}

#[test]
fn clears_tokens_between_lines() {
    let mut lexer = Lexer::new(String::from("unused.txt"));
    lexer.tokenize_line(&String::from("A+B")).unwrap();
    lexer.tokenize_line(&String::from("C")).unwrap();
    assert_kinds(&lexer.tokens, &[TokenKind::Identifier('C')]);
    lexer.tokenize_line(&String::from("# comment")).unwrap();
    assert!(lexer.tokens.is_empty());
}

#[test]
fn formats_errors_with_one_based_positions() {
    assert_eq!(
        LexerError::unexpected_character('@', 2, 4).to_string(),
        "Unexpected character '@' at 3:5"
    );
    assert_eq!(
        LexerError::unexpected_eof(2, 4).to_string(),
        "Unexpected EOF at 3:5"
    );
}

#[test]
fn converts_and_formats_io_errors() {
    let error = LexerError::from(io::Error::new(io::ErrorKind::NotFound, "test missing file"));
    assert_eq!(error.to_string(), "IO error: test missing file");
    assert!(matches!(error, LexerError::Io(ref inner) if inner.kind() == io::ErrorKind::NotFound));
}

// Each file test owns a unique temporary directory. Drop cleans it up even if
// an assertion fails. No extra crate is needed for these small file tests.
struct TestFile {
    directory: std::path::PathBuf,
    path: std::path::PathBuf,
}

impl TestFile {
    fn new(contents: &[u8]) -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "expert-system-{}-{timestamp}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("input.txt");
        let file = Self { directory, path };
        std::fs::write(&file.path, contents).unwrap();
        file
    }

    fn lexer(&self) -> Lexer {
        Lexer::new(self.path.to_str().unwrap().to_string())
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

#[test]
fn tokenizes_file_and_skips_blank_and_comment_lines() {
    let file = TestFile::new(b"# heading\n\nA+B\n   \n?A # query\n");
    let lines = file.lexer().tokenize().unwrap();
    assert_eq!(lines.len(), 2);
    assert_kinds(
        &lines[0],
        &[
            TokenKind::Identifier('A'),
            TokenKind::And,
            TokenKind::Identifier('B'),
        ],
    );
    assert_kinds(&lines[1], &[TokenKind::Query, TokenKind::Identifier('A')]);
    assert_eq!(lines[1][0].pos, 0);
}

#[test]
fn tokenizes_empty_files() {
    for source in ["", "\n# comment\n \t\n"] {
        let file = TestFile::new(source.as_bytes());
        assert!(file.lexer().tokenize().unwrap().is_empty());
    }
}

#[test]
fn handles_crlf_and_missing_final_newline() {
    let file = TestFile::new(b"A\r\n?B");
    let lines = file.lexer().tokenize().unwrap();
    assert_eq!(lines.len(), 2);
    assert_kinds(&lines[0], &[TokenKind::Identifier('A')]);
    assert_kinds(&lines[1], &[TokenKind::Query, TokenKind::Identifier('B')]);
}

#[test]
fn reports_original_line_number_after_skipped_lines() {
    let file = TestFile::new(b"# heading\n\nA\n  @\n");
    assert!(matches!(
        file.lexer().tokenize(),
        Err(LexerError::UnexpectedCharacter {
            character: '@',
            line: 4,
            column: 3,
        })
    ));
}

#[test]
fn returns_io_error_for_missing_file() {
    let file = TestFile::new(b"");
    let mut lexer = Lexer::new(
        file.directory
            .join("missing.txt")
            .to_str()
            .unwrap()
            .to_string(),
    );
    assert!(matches!(lexer.tokenize(), Err(LexerError::Io(ref error))
        if error.kind() == io::ErrorKind::NotFound));
}

#[test]
fn returns_io_error_for_invalid_utf8() {
    let file = TestFile::new(&[0xff]);
    assert!(
        matches!(file.lexer().tokenize(), Err(LexerError::Io(ref error))
        if error.kind() == io::ErrorKind::InvalidData)
    );
}

#[test]
fn equal_points_to_its_first_character() {
    for source in ["=", "=AB", "  = A", "=# empty facts"] {
        let tokens = lex_line(source).unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Equal));
        assert_eq!(tokens[0].pos, source.find('=').unwrap());
    }
}

#[test]
fn equal_leaves_following_tokens_for_normal_scanning() {
    assert_kinds(&lex_line("=# @ ignored").unwrap(), &[TokenKind::Equal]);
    // Grammar validation belongs to the parser, including tokens after '='.
    assert_kinds(
        &lex_line("=!?A").unwrap(),
        &[
            TokenKind::Equal,
            TokenKind::Not,
            TokenKind::Query,
            TokenKind::Identifier('A'),
        ],
    );
}

#[test]
fn reports_first_mismatched_equivalence_character() {
    for (source, expected, column) in [
        ("<A", 'A', 2),
        ("<AB", 'A', 2),
        ("A <== B", '=', 5),
        ("A <> B", '>', 4),
        ("<=#", '#', 3),
    ] {
        assert!(
            matches!(
                lex_line(source),
                Err(LexerError::UnexpectedCharacter { character, line: 1, column: actual })
                    if character == expected && actual == column
            ),
            "input: {source}"
        );
    }
}

#[test]
fn repeated_tokenization_preserves_error_line_numbers() {
    let file = TestFile::new(b"A\n\n@\n");
    let mut lexer = file.lexer();
    for _ in 0..2 {
        assert!(matches!(
            lexer.tokenize(),
            Err(LexerError::UnexpectedCharacter {
                character: '@',
                line: 3,
                column: 1
            })
        ));
    }
    std::fs::write(&file.path, b"A\nB\n").unwrap();
    assert_eq!(lexer.tokenize().unwrap().len(), 2);
    std::fs::write(&file.path, b"@\n").unwrap();
    assert!(matches!(
        lexer.tokenize(),
        Err(LexerError::UnexpectedCharacter {
            character: '@',
            line: 1,
            column: 1,
        })
    ));
}

#[test]
fn uses_character_columns_after_unicode_whitespace() {
    let tokens = lex_line("\u{2003}=A").unwrap();
    assert_eq!(tokens[0].pos, 1);
    assert_eq!(tokens[1].pos, 2);
    assert!(matches!(
        lex_line("\u{2003}<=@"),
        Err(LexerError::UnexpectedCharacter {
            character: '@',
            line: 1,
            column: 4,
        })
    ));
}
