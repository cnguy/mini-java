use crate::lexer::tokens::tag;

#[derive(Debug)]
pub enum Token {
    IdentifierToken { name: String },
    IntegerToken { value: i32 },
    StaticToken { tag: tag::Tag },
    StringToken { value: String },
}

#[allow(dead_code)]
impl Token {
    pub fn to_string(&self) -> &'static str {
        match self {
            Token::IdentifierToken { name: _ } => "IdentifierToken",
            Token::IntegerToken { value: _ } => "IntegerToken",
            Token::StaticToken { tag: _ } => "StaticToken",
            Token::StringToken { value: _ } => "StringToken",
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Token::IdentifierToken { name: _ } => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Token::IntegerToken { value: _ } => true,
            _ => false,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Token::StaticToken { tag: _ } => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Token::StringToken { value: _ } => true,
            _ => false,
        }
    }
}
