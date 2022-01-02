use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Keywords
    #[token("let")]
    Let,

    // Symbols
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("=")]
    Equal,
    #[token("(")]
    LParen,
    #[token("!")]
    Exclam,
    #[token("?")]
    Question,
    #[token(")")]
    RParen,
    #[token(";")]
    Semicolon,

    // Or regular expressions.
    #[regex("[a-zA-Z_]+")]
    Identifier,

    #[regex("[0-9]+")]
    IntegerLiteral,
    #[regex("[0-9]+.[0-9]+")]
    NumberLiteral,
    #[regex("\"([^\"]|\n)*\"")]
    StringLiteral,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn create_lexer(source_code: &str) -> logos::Lexer<'_, Token> {
    Token::lexer(source_code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn let_statement() {
        use crate::{create_lexer,Token};

        let source_code = "let message = \"Hello, world!\"";
        let mut lexer = create_lexer(source_code);

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
        use crate::{create_lexer,Token};

        let source_code = "println!(\"Hello, world!\")";
        let mut lexer = create_lexer(source_code);

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
