use fx_lexer::{Lexer, Token, TokenKind};
use fx_std::fs as fx_std_file;

fn main() {
    println!("Radhey Shyam\n");

    let filepath = "./unused/radha.fx";
    let source = fx_std_file::read_to_string(filepath).unwrap();
    // tokenize(&source)

    let mut lex = Lexer::new(&source);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let token = lex.consume();
        if token.kind == TokenKind::Eof {
            break;
        };
        if token.kind == TokenKind::WhiteSpace {
            continue;
        }
        tokens.push(token);
    }

    dbg!(tokens);
}
