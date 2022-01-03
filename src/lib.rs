mod lexing;

pub type Lexer<'s> = lexing::Lexer<'s>;
pub type Token = lexing::Token;

#[cfg(test)]
mod tests {
    #[test]
    fn let_statement() {
        use crate::{Lexer,Token};

        let source_code = "let message = \"Hello, world!\"";
        let mut lexer = Lexer::new(source_code);

        // let
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.slice(), "let");

        // message
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "message");

        // =
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.slice(), "=");

        // "Hello, world!"
        assert_eq!(lexer.next(), Some(Token::StringLiteral));
        assert_eq!(lexer.slice(), "\"Hello, world!\"");
    }

    #[test]
    fn test_println() {
        use crate::{Lexer,Token};

        let source_code = "println!(\"Hello, world!\")";
        let mut lexer = Lexer::new(source_code);

        // println
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "println");

        // !
        assert_eq!(lexer.next(), Some(Token::Exclam));
        assert_eq!(lexer.slice(), "!");

        // (
        assert_eq!(lexer.next(), Some(Token::LParen));
        assert_eq!(lexer.slice(), "(");

        // "Hello, world!"
        assert_eq!(lexer.next(), Some(Token::StringLiteral));
        assert_eq!(lexer.slice(), "\"Hello, world!\"");

        // )
        assert_eq!(lexer.next(), Some(Token::RParen));
        assert_eq!(lexer.slice(), ")");
    }
}
