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
    characters: Vec<char>,
    position: u32,
    current: char,
    end: bool,
    diagnostics: Diagnostics,
}

impl Lexer {
    pub fn make(source: &'static str, diagnostics: Diagnostics) -> Lexer {
        let characters = source.chars().collect();
        let position = 0;
        let current = '\0';
        let end = false;

        let mut lexer = Lexer {
            characters,
            position,
            current,
            end,
            diagnostics,
        };

        lexer.read_next();

        return lexer;
    }

    pub fn next(&mut self) -> Token {
        loop {
            self.skip_blanks();
            if self.end {
                return Token::StaticToken { tag: Tag::End };
            }

            println!("current character: {}", self.current);

            if self.current.is_digit(10) {
                return self.read_integer_token();
            }

            if self.current.is_alphabetic() {
                return self.read_identifier_token();
            }

            if self.current == '\"' {
                return self.read_string_token();
            }

            match self.current {
                _ => {
                    println!("static token");
                    self.read_next();
                }
            };
        }

        return Token::StaticToken { tag: Tag::End };
    }

    fn read_next(&mut self) {
        if self.current != '\0' {
            self.increment_position();
        }
        if (self.position as usize) >= self.characters.len() {
            self.end = true;
        } else {
            self.current = self.characters[self.position as usize];
        }
    }

    fn skip_blanks(&mut self) {
        while !self.end && (self.current == ' ' || self.current == '\n' || self.current == '\r') {
            self.read_next();
        }
    }

    fn read_integer_token(&mut self) -> Token {
        let mut number = 0;
        while !self.end && self.current.is_digit(10) {
            number = (number * 10) + self.current.to_digit(10).unwrap();
            // println!("{}", number);
            self.read_next();
        }
        println!("return integer token");
        return Token::IntegerToken {
            value: (number as i32),
        };
    }

    fn read_identifier_token(&mut self) -> Token {
        while self.current.is_alphabetic() {
            self.read_next();
        }
        return Token::IdentifierToken {
            name: "empty identifier",
        };
    }

    fn read_string_token(&mut self) -> Token {
        while self.current != '\"' {
            self.read_next();
        }
        return Token::StringToken {
            value: "empty string",
        };
    }

    fn increment_position(&mut self) {
        self.position += 1;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use crate::lexer::tokens::tag::Tag;
    use crate::lexer::tokens::token::Token;

    #[test]
    fn test_playground_1() {
        let source = "class A { }";
        let diagnostics = Diagnostics::make();
        let mut lexer = Lexer::make(source, diagnostics);
        assert_static_token(Tag::Class, lexer.next());
        assert_identifier_token("A", lexer.next());
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_string_token() {
        let source = "\"abc\"";
        let diagnostics = Diagnostics::make();
        let mut lexer = Lexer::make(source, diagnostics);
        assert_string_token("abc", lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_integer_token() {
        let source = "123";
        let diagnostics = Diagnostics::make();
        let mut lexer = Lexer::make(source, diagnostics);
        assert_integer_token(123, lexer.next());
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.diagnostics.has_no_errors());
    }

    #[test]
    fn test_block_comments() {
        let source = "class /* Comment */ A { }";
        let diagnostics = Diagnostics::make();
        let mut lexer = Lexer::make(source, diagnostics);
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
        let mut lexer = Lexer::make(source, diagnostics);
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
        let mut lexer = Lexer::make(source, diagnostics);
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
        let mut lexer = Lexer::make(source, diagnostics);
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
        let mut lexer = Lexer::make(source, diagnostics);
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
        let mut lexer = Lexer::make(source, diagnostics);
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
        println!("{} {:?}", expected_value, actual);
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
