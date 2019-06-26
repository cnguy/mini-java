#[derive(Debug)]
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
    Equal, Unequal,
    Greater, GreaterEqual,
    Minus,
    Modulo,
    Not,
    Or,
    Plus,
    Less, LessEqual,
    Times,
    // punctuation
    OpenBrace, CloseBrace,
    OpenBracket, CloseBracket,
    OpenParenthesis, CloseParenthesis,
    Comma,
    Period,
}