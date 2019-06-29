#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tag {
    // specials
    End,
    // keywords
    Class,
    Else,
    Extends,
    If,
    InstanceOf,
    New,
    Return,
    While,
    // operators
    And,
    Assign,
    Divide,
    Equal,
    Unequal,
    Greater,
    GreaterEqual,
    Minus,
    Modulo,
    Not,
    Or,
    Plus,
    Less,
    LessEqual,
    Times,
    // punctuation
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    Period,
    Semicolon,
}

impl Tag {
    pub fn is_keyword_from_string(identifier: &'static str) -> bool {
        return match identifier {
            "class" => true,
            "else" => true,
            "extends" => true,
            "if" => true,
            "instanceof" => true,
            "new" => true,
            "return" => true,
            "while" => true,
            _ => false,
        };
    }

    // is_keyword documents clearly what's a keyword. :)
    #[warn(unused_code)]
    pub fn is_keyword(&self) -> bool {
        return match self {
            Tag::Class => true,
            Tag::Else => true,
            Tag::Extends => true,
            Tag::If => true,
            Tag::InstanceOf => true,
            Tag::New => true,
            Tag::Return => true,
            Tag::While => true,
            _ => false,
        };
    }
}