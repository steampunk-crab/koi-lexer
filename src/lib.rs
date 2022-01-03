mod lexing;

pub type Token = lexing::Token;
pub type TokenType = lexing::TokenType;

pub fn lex(source_code: String) -> Vec<Token> {
    lexing::lex(source_code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn let_statement() {
        use crate::{lex,TokenType};
        use std::fs;

        // let source_code = "let message = \"Hello, world!\"";
        let source_code =
                fs::read_to_string("samples/let.koi")
                    .expect("An error occured reading the file.");

        let tokens = lex(source_code);

        // let
        assert_eq!(tokens[0].get_type(), TokenType::Let);
        assert_eq!(tokens[0].to_string(), "let");

        // message
        assert_eq!(tokens[1].get_type(), TokenType::Identifier);
        assert_eq!(tokens[1].to_string(), "message");

        // =
        assert_eq!(tokens[2].get_type(), TokenType::Equal);
        assert_eq!(tokens[2].to_string(), "=");

        // "Hello, world!"
        assert_eq!(tokens[3].get_type(), TokenType::StringLiteral);
        assert_eq!(tokens[3].to_string(), "\"Hello, world!\"");
    }

    #[test]
    fn test_println() {
        use crate::{lex,TokenType};

        let source_code = String::from("println!(\"Hello, world!\")");

        let tokens = lex(source_code);

        // println
        assert_eq!(tokens[0].get_type(), TokenType::Identifier);
        assert_eq!(tokens[0].to_string(), "println");

        // !
        assert_eq!(tokens[1].get_type(), TokenType::Exclam);
        assert_eq!(tokens[1].to_string(), "!");

        // (
        assert_eq!(tokens[2].get_type(), TokenType::LParen);
        assert_eq!(tokens[2].to_string(), "(");

        // "Hello, world!"
        assert_eq!(tokens[3].get_type(), TokenType::StringLiteral);
        assert_eq!(tokens[3].to_string(), "\"Hello, world!\"");

        // )
        assert_eq!(tokens[4].get_type(), TokenType::RParen);
        assert_eq!(tokens[4].to_string(), ")");
    }
}
