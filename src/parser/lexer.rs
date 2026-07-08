use std::{iter::Peekable, str::Chars, iter::Iterator};



enum Token {
    Keyword(Keyword),
    Identifier(String),
    Number(String),
    String(String),

    Dot,
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenBracket,
    CloseBracket,

    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Plus,
    Minus,
    Slash,
    Star,
    Percent,

    True,
    False,

    Null,
}


enum Keyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
}

struct Lexer<'a> {
    text: Peekable<Chars<'a>>,
}

impl Iterator for Lexer<'_> {   
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        
    }
}


impl<'a> Lexer<'a> {

}

