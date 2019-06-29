use crate::lexer::tokens::tag::Tag;
use crate::lexer::tokens::token::Token;

pub struct Diagnostics {
    errors: Vec<&'static str>, // static str to keep it simple
}

impl Diagnostics {
    pub fn make() -> Diagnostics {
        return Diagnostics { errors: Vec::new() };
    }

    pub fn has_no_errors(&self) -> bool {
        return self.errors.is_empty();
    }
}

pub struct Lexer {
    source: &'static str,
    position: u32,
    current: char,
    end: bool,
    diagnostics: Diagnostics,
}

impl Lexer {
    pub fn make(source: &'static str, diagnostics: Diagnostics) -> Lexer {
        let position = 0;
        let current = '\0';
        let end = false;

        return Lexer {
            source,
            position,
            current,
            end,
            diagnostics,
        };
    }

    pub fn tokenize(&self, source: &'static str, diagnostics: Diagnostics) {}

    pub fn next(&self) -> Token {
        return Token::StaticToken { tag: Tag::End };
    }

    pub fn run(&self) {
        let int_tok = Token::IntegerToken { value: 0 };
        println!("{}", int_tok.to_string());
        println!("{:?}", int_tok);
        // let static_tok = tokens::static_token::StaticToken { tag: tokens::tag::Tag.Class };
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use crate::lexer::tokens::tag::Tag;
    use crate::lexer::tokens::token::Token;

    #[test]
    fn test_block_comments() {
        let source = "class /* Comment */ A { }";
        let diagnostics = Diagnostics::make();
        let lexer = Lexer::make(source, diagnostics);
        assert_static_token(Tag::Class, lexer.next());
        assert_identifier_token("A", lexer.next());
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_line_comments() {
        let source = "class // Comment \r\nA { }";
        let diagnostics = Diagnostics::make();
        let lexer = Lexer::make(source, diagnostics);
        assert_static_token(Tag::Class, lexer.next());
        assert_identifier_token("A", lexer.next());
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_all_keywords() {
        let source = "class else extends if instanceof new return while";
        let diagnostics = Diagnostics::make();
        let lexer = Lexer::make(source, diagnostics);
        assert_static_token(Tag::Class, lexer.next());
        assert_static_token(Tag::Else, lexer.next());
        assert_static_token(Tag::Extends, lexer.next());
        assert_static_token(Tag::If, lexer.next());
        assert_static_token(Tag::InstanceOf, lexer.next());
        assert_static_token(Tag::New, lexer.next());
        assert_static_token(Tag::Return, lexer.next());
        assert_static_token(Tag::While, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_all_operators() {
        let source = "&& = / == != > >= - % ! || + < <= *";
        let diagnostics = Diagnostics::make();
        let lexer = Lexer::make(source, diagnostics);
        assert_static_token(Tag::And, lexer.next());
        assert_static_token(Tag::Assign, lexer.next());
        assert_static_token(Tag::Divide, lexer.next());
        assert_static_token(Tag::Equal, lexer.next());
        assert_static_token(Tag::Unequal, lexer.next());
        assert_static_token(Tag::Greater, lexer.next());
        assert_static_token(Tag::GreaterEqual, lexer.next());
        assert_static_token(Tag::Minus, lexer.next());
        assert_static_token(Tag::Modulo, lexer.next());
        assert_static_token(Tag::Not, lexer.next());
        assert_static_token(Tag::Or, lexer.next());
        assert_static_token(Tag::Plus, lexer.next());
        assert_static_token(Tag::Less, lexer.next());
        assert_static_token(Tag::LessEqual, lexer.next());
        assert_static_token(Tag::Times, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_all_punctuation() {
        let source = "{}[](),.;";
        let diagnostics = Diagnostics::make();
        let lexer = Lexer::make(source, diagnostics);
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert_static_token(Tag::OpenBracket, lexer.next());
        assert_static_token(Tag::CloseBracket, lexer.next());
        assert_static_token(Tag::Comma, lexer.next());
        assert_static_token(Tag::Period, lexer.next());
        assert_static_token(Tag::Semicolon, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_all_value_tokens() {
        let source = "123 45\"ABC!\"12abc34";
        let diagnostics = Diagnostics::make();
        let lexer = Lexer::make(source, diagnostics);
        assert_integer_token(123, lexer.next());
        assert_integer_token(45, lexer.next());
        assert_string_token("ABC!", lexer.next());
        assert_integer_token(12, lexer.next());
        assert_identifier_token("abc34", lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    fn assert_static_token(expected_tag: Tag, actual: Token) {
        assert!(actual.is_static());

        match actual {
            Token::StaticToken { tag } => assert!(tag == expected_tag),
            _ => (), // useless
        };
    }

    fn assert_identifier_token(expected_identifier: &'static str, actual: Token) {
        assert!(actual.is_identifier());

        match actual {
            Token::IdentifierToken { name } => assert!(expected_identifier == name),
            _ => (), // useless
        }
    }

    fn assert_integer_token(expected_value: i32, actual: Token) {
        assert!(actual.is_integer());

        match actual {
            Token::IntegerToken { value } => assert!(expected_value == value),
            _ => (), // useless
        };
    }

    fn assert_string_token(expected_string: &'static str, actual: Token) {
        assert!(actual.is_string());

        match actual {
            Token::StringToken { value } => assert!(expected_string == value),
            _ => (), // useless
        };
    }
}
