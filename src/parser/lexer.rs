use std::{iter::Peekable, str::Chars, iter::Iterator, fmt::Display};

enum Token {
    Keyword(Keyword),
    Identifier(String),
    Number(String),
    String(String),

    Dot,            // .
    Comma,          // ,
    Semicolon,      // ;
    OpenParen,      // ( 
    CloseParen,     // )
    OpenSquare,     // [
    CloseSquare,    // ] 
    OpenBracket,    // {
    CloseBracket,   // }


    Bang,                   // !
    Equal,                  // =
    Equality,               // ==
    StrictEquality,         // ===
    Inequality,             // !=
    StrictInequality,       // !==
    GreaterThan,            // >
    GreaterThanOrEqual,     // >=
    LessThan,               // <
    LessThanOrEqual,        // <=
    Plus,                   // +
    Minus,                  // -
    Slash,                  // /
    Star,                   // *
    Percent,                // %

    True,
    False,

    Null,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Keyword(k) => return k.fmt(f),
            Self::Identifier(s) => s,
            Self::Number(n) => n,
            Self::String(s) => s,
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenSquare => "[",
            Self::CloseSquare => "]",
            Self::OpenBracket => "{",
            Self::CloseBracket => "}" ,
            Self::Equal => "=" ,
            Self::Equality => "==" ,
            Self::StrictEquality => "===" ,
            Self::Inequality => "!=" ,
            Self::StrictInequality => "!==" ,
            Self::GreaterThan => "" ,
            Self::GreaterThanOrEqual => "" ,
            Self::LessThan => "" ,
            Self::LessThanOrEqual => "" ,
            Self::Plus => "" ,
            Self::Minus => "" ,
            Self::Slash => "" ,
            Self::Star => "" ,
            Self::Percent => "" ,
            Self::True => "" ,
            Self::False => "" ,
            Self::Null => "" ,
        })
    }
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

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Await => "await",
            Self::Break => "break",
            Self::Case => "case",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::Debugger => "debugger",
            Self::Default => "default",
            Self::Delete => "delete",
            Self::Do => "do",
            Self::Else => "else",
            Self::Export => "export",
            Self::Extends => "extends",
            Self::False => "false",
            Self::Finally => "finally",
            Self::For => "for",
            Self::Function => "function",
            Self::If => "if",
            Self::Import => "import",
            Self::In => "in",
            Self::Instanceof => "instanceof",
            Self::New => "new",
            Self::Null => "null",
            Self::Return => "return",
            Self::Super => "super",
            Self::Switch => "switch",
            Self::This => "this",
            Self::Throw => "throw",
            Self::True => "true",
            Self::Try => "try",
            Self::Typeof => "typeof",
            Self::Var => "var",
            Self::Void => "void",
            Self::While => "while",
            Self::With => "with",
        })   
    }
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

