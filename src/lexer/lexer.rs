use crate::lexer::tokens::token::Token;

pub struct Lexer {
    position: u32,
    current: char,
    end: bool,
}

pub fn make() -> Lexer {
    let position = 0;
    let current = '\0';
    let end = false;
    return Lexer {
        position,
        current,
        end,
    };
}

impl Lexer {
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

    use crate::lexer::tokens::tag;
    use crate::lexer::tokens::token;

    #[test]
    fn test_block_comments() {
        // assert_static_token(tag::Tag::End, token::Token::StaticToken { tag: tag::Tag::End })
    }

    #[test]
    fn test_line_comments() {}

    #[test]
    fn test_all_keywords() {}

    #[test]
    fn test_all_operators() {}

    #[test]
    fn test_all_punctuation() {}

    #[test]
    fn test_all_value_tokens() {}

    fn assert_static_token(expectedTag: tag::Tag, actual: token::Token) {
        assert!(actual.is_static());

        match actual {
            Token::StaticToken { tag } => assert!(tag == expectedTag),
            _ => (), // useless
        };
    }

    fn assert_identifier_token(expectedIdentifier: &'static str, actual: token::Token) {
        assert!(actual.is_identifier());

        match actual {
            Token::IdentifierToken { name } => assert!(expectedIdentifier == name),
            _ => (), // useless
        }
    }

    fn assert_integer_token(expectedValue: i32, actual: token::Token) {
        assert!(actual.is_integer());

        match actual {
            Token::IntegerToken { value } => assert!(expectedValue == value),
            _ => (), // useless
        };
    }

    fn assert_string_token(expectedString: &'static str, actual: token::Token) {
        assert!(actual.is_string());

        match actual {
            Token::StringToken { value } => assert!(expectedString == value),
            _ => (), // useless
        };
    }
}
