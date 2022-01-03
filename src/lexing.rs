use logos::Logos;

use std::fmt;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Logos, PartialEq)]
pub enum TokenType {
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

pub struct Token {
    slice: String,
    span: Range<usize>,
    token_type: TokenType,
}

impl Token {
    pub fn get_span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.slice)
    }
}

type LogosLexer<'s> = logos::Lexer<'s, TokenType>;

struct Lexer {
    source_code: String
}

impl Lexer {
    pub fn new(source_code: String) -> Lexer {
        Lexer {
            source_code: source_code
        }
    }

    pub fn lex(&self) -> Vec<Token> {
        // Create a lexer to scan the source code.
        let source_code = self.source_code.as_str();
        let mut logos_lexer = create_logos_lexer(source_code);

        // Lex the source code into a vector of tokens.
        let tokens = logos_lex(&mut logos_lexer);

        tokens
    }
}

impl Clone for Lexer {
    fn clone(&self) -> Self {
        let source_code = self.source_code.clone();

        Lexer {
            source_code: source_code
        }
    }
}

pub fn lex(source_code: String) -> Vec<Token> {
    // Create a lexer from the source code.
    let lexer = Lexer::new(source_code);

    // Lex the source code into a vector of tokens.
    lexer.lex()
}

fn create_logos_lexer<'s>(source_code: &'s str) -> LogosLexer<'_> {
    TokenType::lexer(source_code)
}

fn get_token(logos_lexer: &mut LogosLexer) -> Token {
    let token_type = logos_lexer.next().unwrap_or(TokenType::Error);
    let slice: String;
    let span: Range<usize>;

    slice = String::from(logos_lexer.slice());
    span = logos_lexer.span();

    let token = Token {
        slice: slice,
        span: span,
        token_type: token_type
    };

    token
}

fn logos_lex(logos_lexer: &mut LogosLexer) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Start with any arbitrary type other than Error.
    let mut loop_terminator = TokenType::Identifier;

    // While the type is not Error:
    while loop_terminator != TokenType::Error {
        // Get the next token.
        let token = get_token(logos_lexer);

        // Update the loop terminator.
        loop_terminator = token.get_type();

        // Update the
        tokens.push(token);
    }

    tokens

    // let len = data.c;
    // (0..len).map(move |i| data[i])
}
