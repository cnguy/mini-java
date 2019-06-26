use crate::lexer::tokens::tag;

#[derive(Debug)]
pub enum Token {
    IdentifierToken { name: &'static str },
    IntegerToken { value: i32 },
    StaticToken { tag: tag::Tag },
    StringToken { value: &'static str },
}

impl Token {
    pub fn to_string(&self) -> &'static str { 
        return match self {
            Token::IdentifierToken { name } => "IdentifierToken",
            Token::IntegerToken { value } => "IntegerToken",
            Token::StaticToken { tag } => "StaticToken",
            Token::StringToken { value } => "StringToken",
        }
    }
}
