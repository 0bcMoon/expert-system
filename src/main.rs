use expert_system::Lexer;

fn main() {
    let filename = String::from("examples/input.txt");
    let mut lexer = Lexer::new(filename);
    let m = lexer.tokenize();

    let m = match m {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(o) => o,
    };
    for n in m {
        dbg!(n);
    }
}
/*

E  -> TE'
E' -> +TE' | eps
T  -> FT'
T' -> * FT' | eps
F -> num | (E)





*/
