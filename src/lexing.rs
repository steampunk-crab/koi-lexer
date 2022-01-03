use logos::Logos;

pub struct Lexer<'s> {
    lexer: logos::Lexer<'s, Token>,
    source_code: &'s str
}

impl<'s> Lexer<'s> {
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            lexer: create_lexer(source_code),
            source_code: source_code
        }
    }

    pub fn get_source_code(&self) -> &str {
        self.source_code
    }

    pub fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }
}

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

fn create_lexer(source_code: &str) -> logos::Lexer<'_, Token> {
    Token::lexer(source_code)
}
