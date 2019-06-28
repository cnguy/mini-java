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
        };
    }

    pub fn is_identifier(&self) -> bool {
        return match self {
            Token::IdentifierToken { name } => true,
            _ => false,
        };
    }

    pub fn is_integer(&self) -> bool {
        return match self {
            Token::IntegerToken { value } => true,
            _ => false,
        };
    }

    pub fn is_static(&self) -> bool {
        return match self {
            Token::StaticToken { tag } => true,
            _ => false,
        };
    }
    pub fn is_string(&self) -> bool {
        return match self {
            Token::StringToken { value } => true,
            _ => false,
        };
    }
}
