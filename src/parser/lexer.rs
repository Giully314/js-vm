use std::{iter::Peekable, str::Chars, iter::Iterator, fmt::Display};

use crate::errors::{Error, Result};

#[derive(PartialEq, Debug)]
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
            Self::Bang => "!" ,
            Self::Equal => "=" ,
            Self::Equality => "==" ,
            Self::StrictEquality => "===" ,
            Self::Inequality => "!=" ,
            Self::StrictInequality => "!==" ,
            Self::GreaterThan => "" ,
            Self::GreaterThanOrEqual => "" ,
            Self::LessThan => "" ,
            Self::LessThanOrEqual => "" ,
            Self::Plus => "+" ,
            Self::Minus => "-" ,
            Self::Slash => "/" ,
            Self::Star => "*" ,
            Self::Percent => "%" ,
        })
    }
}

#[derive(PartialEq, Debug)]
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
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => self.text.peek().map(|c| Err(Error::InvalidInput(format!("unexpected char {c}")))),
            Err(err) => Some(Err(err))
        }
    }
}


impl<'a> Lexer<'a> {
    fn new(text: &'a str) -> Self {
        return Lexer { text: text.chars().peekable() };
    }

    pub fn scan(&mut self) -> Result<Option<Token>> {
        self.skip_whitespaces();

        let Some(c) = self.text.peek() else {
            return Ok(None);
        };

        match c {
            '0'..'9' => Ok(self.number()),
            'a'..'z' | 'A'..'Z' => {
                return Ok(self.keyword_or_identifier());
            },
            '"' => Ok(self.string()),
            _ => self.symbol()
        }
    }

    fn symbol(&mut self) -> Result<Option<Token>> {
        let c =  self.text.next().unwrap();
        match c {
            '.' => Ok(Some(Token::Dot)),
            ',' => Ok(Some(Token::Comma)),
            ';' => Ok(Some(Token::Semicolon)),
            '('=> Ok(Some(Token::OpenParen)),
            ')'=> Ok(Some(Token::CloseParen)),
            '['=> Ok(Some(Token::OpenSquare)),
            ']'=> Ok(Some(Token::CloseSquare)),
            '{'=> Ok(Some(Token::OpenBracket)),
            '}'=> Ok(Some(Token::CloseBracket)),
            '=' => {
                self.text.next();
                if self.match_char('=') {
                    if self.match_char('=') {
                        return Ok(Some(Token::StrictEquality));
                    }
                    return Ok(Some(Token::Equality));
                }
                return Ok(Some(Token::Equal));
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        return Ok(Some(Token::StrictInequality));
                    } 
                    return Ok(Some(Token::Inequality));
                }
                return Ok(Some(Token::Bang)); 
            } 
            '>' => {
                if self.match_char('=') {
                    return Ok(Some(Token::GreaterThanOrEqual));
                }
                return Ok(Some(Token::GreaterThan));
            }
            '<' => {
                if self.match_char('=') {
                    return Ok(Some(Token::LessThanOrEqual));
                }
                return Ok(Some(Token::LessThan));
            },
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '/' => Ok(Some(Token::Slash)), // TODO: Add comment skippin.
            '*' => Ok(Some(Token::Star)),
            '%' => Ok(Some(Token::Percent)),
            _ => Err(Error::InvalidInput(c.to_string())),
        }
    }

    // Returns the next character if the predicate is satisfied by it.
    fn next_if(&mut self, p: impl Fn(char) -> bool) -> Option<char> {
        self.text.peek().filter(|&&c| p(c))?;
        self.text.next()
    }

    // If the current char match the expected, advance and return true, otherwise false. 
    fn match_char(&mut self, expected: char) -> bool {
        self.next_if(|c| c == expected).is_some()
    }

    // Skip all whitespaces.
    fn skip_whitespaces(&mut self) {
        while self.next_if(|c| c.is_whitespace()).is_some() {}
    }

    fn number(&mut self) -> Option<Token> {
        // TODO: Check for malforming numbers (mix of numbers and alpha).
        let mut n = String::new();
        while let Some(c) = self.next_if(|c| c.is_numeric()) {
            n.push(c);
        }

        if self.match_char('.') {
            n.push('.');

            while let Some(c) = self.next_if(|c| c.is_numeric()) {
                n.push(c);
            }
        }

        return Some(Token::Number(n))
    }

    fn string(&mut self) -> Option<Token> {
        self.text.next();
        let mut s = String::new();
        while let Some(c) = self.next_if(|c| c != '"') {
            s.push(c);
        }
        self.text.next();
        return Some(Token::String(s));
    }

    fn keyword_or_identifier(&mut self) -> Option<Token> {
        // This is not optimal, i know. Best way is to just take a &str as a text and indexing 
        // directly for the trie. 
        let mut s = String::new();
        while let Some(c) = self.next_if(|c| c.is_alphanumeric() || c == '_') {
            s.push(c);
        }

        match s.as_str() {
            "await" => Some(Token::Keyword(Keyword::Await)),
            "break" => Some(Token::Keyword(Keyword::Break)),
            "case" => Some(Token::Keyword(Keyword::Case)),
            "catch" => Some(Token::Keyword(Keyword::Catch)),
            "class" => Some(Token::Keyword(Keyword::Class)),
            "const" => Some(Token::Keyword(Keyword::Const)),
            "continue" => Some(Token::Keyword(Keyword::Continue)),
            "debugger" => Some(Token::Keyword(Keyword::Debugger)),
            "default" => Some(Token::Keyword(Keyword::Default)),
            "delete" => Some(Token::Keyword(Keyword::Delete)),
            "do" => Some(Token::Keyword(Keyword::Do)),
            "else" => Some(Token::Keyword(Keyword::Else)),
            "export" => Some(Token::Keyword(Keyword::Export)),
            "extends" => Some(Token::Keyword(Keyword::Extends)),
            "false" => Some(Token::Keyword(Keyword::False)),
            "finally" => Some(Token::Keyword(Keyword::Finally)),
            "for" => Some(Token::Keyword(Keyword::For)),
            "function" => Some(Token::Keyword(Keyword::Function)),
            "if" => Some(Token::Keyword(Keyword::If)),
            "import" => Some(Token::Keyword(Keyword::Import)),
            "in" => Some(Token::Keyword(Keyword::In)),
            "instanceof" => Some(Token::Keyword(Keyword::Instanceof)),
            "new" => Some(Token::Keyword(Keyword::New)),
            "null" => Some(Token::Keyword(Keyword::Null)),
            "return" => Some(Token::Keyword(Keyword::Return)),
            "super" => Some(Token::Keyword(Keyword::Super)),
            "switch" => Some(Token::Keyword(Keyword::Switch)),
            "this" => Some(Token::Keyword(Keyword::This)),
            "throw" => Some(Token::Keyword(Keyword::Throw)),
            "true" => Some(Token::Keyword(Keyword::True)),
            "try" => Some(Token::Keyword(Keyword::Try)),
            "typeof" => Some(Token::Keyword(Keyword::Typeof)),
            "var" => Some(Token::Keyword(Keyword::Var)),
            "void" => Some(Token::Keyword(Keyword::Void)),
            "while" => Some(Token::Keyword(Keyword::While)),
            "with" => Some(Token::Keyword(Keyword::With)),
            _ => Some(Token::Identifier(s))
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_numbers() {
        let mut lexer = Lexer::new("42");
        assert_eq!(lexer.next(), Some(Ok(Token::Number("42".to_string()))));
    }
}