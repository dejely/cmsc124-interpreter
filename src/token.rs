#[derive(Debug, Clone, PartialEq)] 

    pub enum TokenType {
        
        // Single-character tokens.
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        LeftBracket,
        RightBracket,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,

        // One or two character tokens.
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        // Literals.
        Identifier,
        String,
        Number,

        // Keywords.
        And,
        Class,
        Else,
        False,
        Fun,
        For,
        If,
        
        // End of Input.
        Eof,
        }

    pub struct Token {
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    }

    pub enum Literal {
        Number(f64),
        String(String),
        Boolean(bool),
        Null,
        
    }