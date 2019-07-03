#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
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

#[allow(dead_code)]
impl Tag {
    pub fn is_keyword_from_string(identifier: &String) -> bool {
        match identifier.as_ref() {
            "class" => true,
            "else" => true,
            "extends" => true,
            "if" => true,
            "instanceof" => true,
            "new" => true,
            "return" => true,
            "while" => true,
            _ => false,
        }
    }

    pub fn to_keyword_from_string(identifier: &String) -> Tag {
        match identifier.as_ref() {
            "class" => Tag::Class,
            "else" => Tag::Else,
            "extends" => Tag::Extends,
            "if" => Tag::If,
            "instanceof" => Tag::InstanceOf,
            "new" => Tag::New,
            "return" => Tag::Return,
            "while" => Tag::While,
            _ => Tag::End,
        }
    }

    // is_keyword documents clearly what's a keyword. :)
    pub fn is_keyword(&self) -> bool {
        match self {
            Tag::Class => true,
            Tag::Else => true,
            Tag::Extends => true,
            Tag::If => true,
            Tag::InstanceOf => true,
            Tag::New => true,
            Tag::Return => true,
            Tag::While => true,
            _ => false,
        }
    }
}