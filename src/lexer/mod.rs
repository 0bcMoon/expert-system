use std::fmt;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

// Compile this module only when running tests.
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum TokenKind {
    Implies,
    Query,
    Not,
    And,
    Or,
    Xor,
    IfAndOnlyIf,
    Oparenth,
    Cparenth,
    Identifier(char),
    Equal,
    Eof,
}

type LexerResult = Result<(), LexerError>;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}
#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter {
        character: char,
        line: usize,
        column: usize,
    },
    UnexpectedEof {
        line: usize,
        column: usize,
    },
    Io(io::Error),
}

impl LexerError {
    /// Convert zero-based line and column indices to one-based error positions.
    pub fn unexpected_character(character: char, line: usize, column: usize) -> Self {
        Self::UnexpectedCharacter {
            character,
            line: line + 1,
            column: column + 1,
        }
    }

    /// Report incomplete input, converting zero-based indices to one-based positions.
    pub fn unexpected_eof(line: usize, column: usize) -> Self {
        Self::UnexpectedEof {
            line: line + 1,
            column: column + 1,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter {
                character,
                line,
                column,
            } => {
                write!(f, "Unexpected character '{character}' at {line}:{column}")
            }

            Self::UnexpectedEof { line, column } => {
                write!(f, "Unexpected EOF at {line}:{column}")
            }

            Self::Io(err) => {
                write!(f, "IO error: {err}")
            }
        }
    }
}

impl From<std::io::Error> for LexerError {
    fn from(err: std::io::Error) -> Self {
        LexerError::Io(err)
    }
}
#[derive(Debug)]
pub struct Lexer {
    filename: String,
    idx: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(filename: String) -> Self {
        Lexer {
            filename,
            idx: 0,
            line: 0,
            tokens: vec![],
        }
    }

    fn read_file(&mut self) -> io::Result<Vec<String>> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        reader.lines().collect()
    }

    fn tokenize_line(&mut self, line: &str) -> LexerResult {
        let line: Vec<char> = line.chars().collect();
        self.idx = 0;
        self.tokens.clear();

        while self.idx < line.len() {
            let c = line[self.idx];
            match c {
                c if c.is_whitespace() => self.lex_whitespace(),
                '#' => break,
                'a'..='z' | 'A'..='Z' => self.lex_ident(c),
                '+' | '|' | '^' | '!' | '?' | '(' | ')' => self.lax_operation(c),
                '=' => self.lex_equal(&line),
                '<' => self.lex_if_and_only_if(&line[self.idx..]),
                other => Err(LexerError::unexpected_character(other, self.line, self.idx)),
            }?;
        }
        Ok(())
    }

    fn lex_whitespace(&mut self) -> LexerResult {
        self.idx += 1;
        Ok(())
    }

    fn lex_ident(&mut self, c: char) -> LexerResult {
        self.tokens.push(Token {
            kind: TokenKind::Identifier(c),
            pos: self.idx,
        });
        self.idx += 1;
        Ok(())
    }

    fn lax_operation(&mut self, c: char) -> LexerResult {
        let token_kind = match c {
            '+' => TokenKind::And,
            '|' => TokenKind::Or,
            '^' => TokenKind::Xor,
            '!' => TokenKind::Not,
            '?' => TokenKind::Query,
            '(' => TokenKind::Oparenth,
            ')' => TokenKind::Cparenth,
            _ => unreachable!(),
        };

        self.tokens.push(Token {
            kind: token_kind,
            pos: self.idx,
        });
        self.idx += 1;
        Ok(())
    }

    fn lex_equal(&mut self, line: &[char]) -> LexerResult {
        let eq_pos = self.idx;
        self.idx += 1;

        if line.get(self.idx) == Some(&'>') {
            return self.lex_implies();
        }

        self.tokens.push(Token {
            kind: TokenKind::Equal,
            pos: eq_pos,
        });
        Ok(())
    }

    fn lex_implies(&mut self) -> LexerResult {
        self.tokens.push(Token {
            kind: TokenKind::Implies,
            pos: self.idx - 1,
        });
        self.idx += 1;
        Ok(())
    }

    fn lex_if_and_only_if(&mut self, line: &[char]) -> LexerResult {
        let if_slice: &[char] = &['<', '=', '>'];

        for (offset, expected) in if_slice.iter().enumerate() {
            let Some(&actual) = line.get(offset) else {
                return Err(LexerError::unexpected_eof(self.line, self.idx + offset));
            };
            if actual != *expected {
                return Err(LexerError::unexpected_character(
                    actual,
                    self.line,
                    self.idx + offset,
                ));
            }
        }

        self.tokens.push(Token {
            kind: TokenKind::IfAndOnlyIf,
            pos: self.idx,
        });
        self.idx += if_slice.len();
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Vec<Token>>, LexerError> {
        self.line = 0;
        self.idx = 0;
        self.tokens.clear();
        let lines = self.read_file()?;
        let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
        for line in lines {
            self.tokenize_line(&line)?;
            if !self.tokens.is_empty() {
                tokenized_lines.push(std::mem::take(&mut self.tokens));
            }
            self.line += 1;
        }
        Ok(tokenized_lines)
    }
}
