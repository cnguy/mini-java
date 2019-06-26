use crate::lexer::tokens::token::Token;

pub struct Lexer {
    position: u32,
    current: char,
    end: bool
}

pub fn make() -> Lexer {
    let position = 0;
    let current = '\0';
    let end = false;
    return Lexer { position, current, end };
}

impl Lexer {
    pub fn run(&self) {
        let int_tok = Token::IntegerToken { value: 0 };
        println!("{}", int_tok.to_string());
        println!("{:?}", int_tok);
        // let static_tok = tokens::static_token::StaticToken { tag: tokens::tag::Tag.Class };
    }
}
