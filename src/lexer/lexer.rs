use crate::shared::diagnostics::Diagnostics;

use crate::lexer::tokens::tag::Tag;
use crate::lexer::tokens::token::Token;

#[allow(dead_code)]
pub struct Lexer {
    characters: Vec<char>,
    position: u32,
    current: char,
    end: bool,
    diagnostics: Diagnostics,
}

#[allow(dead_code)]
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

        lexer
    }

    pub fn is_clean(&self) -> bool {
        self.diagnostics.has_no_errors()
    }

    pub fn next(&mut self) -> Token {
        self.skip_blanks();

        println!("current character: {}", self.current);

        match self.current {
            _ if self.end => Token::StaticToken { tag: Tag::End },
            _ if self.current.is_digit(10) => self.read_integer_token(),
            _ if self.current.is_alphabetic() => self.read_identifier_token(),
            _ if self.current == '"' => self.read_string_token(),
            '&' => {
                self.read_next();
                if self.end || self.current != '&' {
                    self.diagnostics.report("must be &&");
                    Token::StaticToken { tag: Tag::End }
                } else {
                    println!("return and");
                    self.read_next();
                    Token::StaticToken { tag: Tag::And }
                }
            }
            '=' => {
                self.read_next();
                if self.current == '=' {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Equal }
                } else {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Assign }
                }
            }
            '/' => {
                self.read_next();
                match self.current {
                    '/' => self.skip_line_comment(),
                    '*' => self.skip_block_comment(),
                    _ => Token::StaticToken { tag: Tag::Divide },
                }
            }
            '!' => {
                self.read_next();
                if self.current == '=' {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Unequal }
                } else {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Not }
                }
            }
            '|' => {
                self.read_next();
                if self.end || self.current != '|' {
                    self.diagnostics.report("must be ||");
                    Token::StaticToken { tag: Tag::End }
                } else {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Or }
                }
            }
            '>' => {
                self.read_next();
                if self.current == '=' {
                    self.read_next();
                    Token::StaticToken {
                        tag: Tag::GreaterEqual,
                    }
                } else {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Greater }
                }
            }
            '<' => {
                self.read_next();
                if self.current == '=' {
                    self.read_next();
                    Token::StaticToken {
                        tag: Tag::LessEqual,
                    }
                } else {
                    self.read_next();
                    Token::StaticToken { tag: Tag::Less }
                }
            }
            '-' => {
                self.read_next();
                Token::StaticToken { tag: Tag::Minus }
            }
            '%' => {
                self.read_next();
                Token::StaticToken { tag: Tag::Modulo }
            }
            '+' => {
                self.read_next();
                Token::StaticToken { tag: Tag::Plus }
            }
            '*' => {
                self.read_next();
                Token::StaticToken { tag: Tag::Times }
            }
            '{' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::OpenBrace,
                }
            }
            '}' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::CloseBrace,
                }
            }
            '[' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::OpenBracket,
                }
            }
            ']' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::CloseBracket,
                }
            }
            '(' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::OpenParenthesis,
                }
            }
            ')' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::CloseParenthesis,
                }
            }
            ',' => {
                self.read_next();
                Token::StaticToken { tag: Tag::Comma }
            }
            '.' => {
                self.read_next();
                Token::StaticToken { tag: Tag::Period }
            }
            ';' => {
                self.read_next();
                Token::StaticToken {
                    tag: Tag::Semicolon,
                }
            }
            _ => {
                println!("static token");
                self.read_next();
                Token::StaticToken { tag: Tag::End }
            }
        }
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

        Token::IntegerToken {
            value: (number as i32),
        }
    }

    fn read_identifier_token(&mut self) -> Token {
        let mut name_or_identifier = String::new();

        while !self.end && self.current.is_alphanumeric() {
            name_or_identifier.push(self.current);
            self.read_next();
        }

        if Tag::is_keyword_from_string(&name_or_identifier) {
            let tag = Tag::to_keyword_from_string(&name_or_identifier);
            Token::StaticToken { tag }
        } else {
            Token::IdentifierToken {
                name: name_or_identifier.clone(),
            }
        }
    }

    fn read_string_token(&mut self) -> Token {
        self.read_next(); // Get past the current ".

        let mut value = String::new();

        while !self.end && self.current != '"' {
            value.push(self.current);
            self.read_next();
        }

        if self.current != '"' {
            self.diagnostics.report("non-terminated string");
            return Token::StaticToken { tag: Tag::End };
        }

        self.read_next();

        Token::StringToken { value }
    }

    fn skip_line_comment(&mut self) -> Token {
        self.read_next();

        while !self.end && (self.current != '\n' && self.current != '\r') {
            self.read_next();
        }

        self.read_next();

        self.next()
    }

    fn skip_block_comment(&mut self) -> Token {
        self.read_next();

        while !self.end && self.current != '*' {
            self.read_next();
        }

        if self.current != '*' {
            self.diagnostics
                .report("unterminated block comment. without */");
            return Token::StaticToken { tag: Tag::End };
        }

        self.read_next();

        if self.current != '/' {
            self.diagnostics
                .report("unterminated block comment. without /");
            return Token::StaticToken { tag: Tag::End };
        }

        self.read_next();

        self.next()
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

    fn lexer(source: &'static str) -> Lexer {
        let diagnostics = Diagnostics::make();
        Lexer::make(source, diagnostics)
    }

    #[test]
    fn test_playground_1() {
        let mut lexer = lexer("class A { }");
        assert_static_token(Tag::Class, lexer.next());
        assert_identifier_token("A", lexer.next());
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_string_token() {
        let mut lexer = lexer("\"abc\"");
        assert_string_token("abc", lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_unterminated_string_token() {
        let mut lexer = lexer("\"abc");
        assert_static_token(Tag::End, lexer.next());
        assert!(!lexer.is_clean());
    }

    #[test]
    fn test_single_ampersand() {
        let mut lexer = lexer("&");
        assert_static_token(Tag::End, lexer.next());
        assert!(!lexer.is_clean());
    }

    #[test]
    fn test_single_line() {
        let mut lexer = lexer("|");
        assert_static_token(Tag::End, lexer.next());
        assert!(!lexer.is_clean());
    }

    #[test]
    fn test_unterminated_block_comment_1() {
        let mut lexer = lexer("/*");
        assert_static_token(Tag::End, lexer.next());
        assert!(!lexer.is_clean());
    }

    #[test]
    fn test_unterminated_block_comment_2() {
        let mut lexer = lexer("/**");
        assert_static_token(Tag::End, lexer.next());
        assert!(!lexer.is_clean());
    }

    #[test]
    fn test_simple_block_comment() {
        let mut lexer = lexer("/**/");
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_integer_token() {
        let mut lexer = lexer("123");
        assert_integer_token(123, lexer.next());
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_block_comments() {
        let mut lexer = lexer("class /* Comment */ A { }");
        assert_static_token(Tag::Class, lexer.next());
        assert_identifier_token("A", lexer.next());
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert_static_token(Tag::End, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_line_comments() {
        let mut lexer = lexer("class // Comment \r\nA { }");
        assert_static_token(Tag::Class, lexer.next());
        assert_identifier_token("A", lexer.next());
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_all_keywords() {
        let mut lexer = lexer("class else extends if instanceof new return while");
        assert_static_token(Tag::Class, lexer.next());
        assert_static_token(Tag::Else, lexer.next());
        assert_static_token(Tag::Extends, lexer.next());
        assert_static_token(Tag::If, lexer.next());
        assert_static_token(Tag::InstanceOf, lexer.next());
        assert_static_token(Tag::New, lexer.next());
        assert_static_token(Tag::Return, lexer.next());
        assert_static_token(Tag::While, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_all_operators() {
        let mut lexer = lexer("&& = / == != > >= - % ! || + < <= *");
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
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_all_punctuation() {
        let mut lexer = lexer("{}[](),.;");
        assert_static_token(Tag::OpenBrace, lexer.next());
        assert_static_token(Tag::CloseBrace, lexer.next());
        assert_static_token(Tag::OpenBracket, lexer.next());
        assert_static_token(Tag::CloseBracket, lexer.next());
        assert_static_token(Tag::OpenParenthesis, lexer.next());
        assert_static_token(Tag::CloseParenthesis, lexer.next());
        assert_static_token(Tag::Comma, lexer.next());
        assert_static_token(Tag::Period, lexer.next());
        assert_static_token(Tag::Semicolon, lexer.next());
        assert!(lexer.is_clean());
    }

    #[test]
    fn test_all_value_tokens() {
        let mut lexer = lexer("123 45\"ABC!\"12abc34");
        assert_integer_token(123, lexer.next());
        assert_integer_token(45, lexer.next());
        assert_string_token("ABC!", lexer.next());
        assert_integer_token(12, lexer.next());
        println!("{:?}", lexer.diagnostics.errors);
        assert_identifier_token("abc34", lexer.next());
        assert!(lexer.is_clean());
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
            Token::IdentifierToken { name } => assert_eq!(expected_identifier, name),
            _ => (), // useless
        };
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
            Token::StringToken { value } => assert_eq!(expected_string, value),
            _ => (), // useless
        };
    }
}
