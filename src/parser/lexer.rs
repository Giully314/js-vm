use std::{iter::Peekable, str::Chars, iter::Iterator, fmt::Display};

#[derive(PartialEq, Debug)]
pub enum Token {
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

    Eof,
    Error(String),
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
            Self::Eof => "eof" ,
            Self::Error(s) => s,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
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

pub struct Lexer<'a> {
    text: Peekable<Chars<'a>>,
}

impl Iterator for Lexer<'_> {   
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan() {
            Token::Eof => None,
            token => Some(token),
        }
    }
}


impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        return Lexer { text: text.chars().peekable() };
    }

    pub fn scan(&mut self) -> Token {
        self.skip_whitespaces();

        let Some(c) = self.text.peek() else {
            return Token::Eof;
        };

        match c {
            '0'..'9' => self.number(),
            'a'..'z' | 'A'..'Z' => {
                return self.keyword_or_identifier();
            },
            '"' => self.string(),
            _ => self.symbol()
        }
    }

    fn symbol(&mut self) -> Token {
        let Some(c) =  self.text.next() else {
            return Token::Eof;
        };

        match c {
            '.' => Token::Dot,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '('=> Token::OpenParen,
            ')'=> Token::CloseParen,
            '['=> Token::OpenSquare,
            ']'=> Token::CloseSquare,
            '{'=> Token::OpenBracket,
            '}'=> Token::CloseBracket,
            '=' => {
                self.text.next();
                if self.match_char('=') {
                    if self.match_char('=') {
                        return Token::StrictEquality;
                    }
                    return Token::Equality;
                }
                return Token::Equal;
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        return Token::StrictInequality;
                    } 
                    return Token::Inequality;
                }
                return Token::Bang; 
            } 
            '>' => {
                if self.match_char('=') {
                    return Token::GreaterThanOrEqual;
                }
                return Token::GreaterThan;
            }
            '<' => {
                if self.match_char('=') {
                    return Token::LessThanOrEqual;
                }
                return Token::LessThan;
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash, // TODO: Add comment skippin.
            '*' => Token::Star,
            '%' => Token::Percent,
            _ => Token::Error(format!("Invalid character {c}")),
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

    fn number(&mut self) -> Token {
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

        return Token::Number(n)
    }

    fn string(&mut self) -> Token {
        self.text.next();
        let mut s = String::new();
        while let Some(c) = self.next_if(|c| c != '"') {
            s.push(c);
        }
        self.text.next();
        return Token::String(s);
    }

    fn keyword_or_identifier(&mut self) -> Token {
        // This is not optimal, i know. Best way is to just take a &str as a text and indexing 
        // directly for the trie. 
        let mut s = String::new();
        while let Some(c) = self.next_if(|c| c.is_alphanumeric() || c == '_') {
            s.push(c);
        }

        match s.as_str() {
            "await" => Token::Keyword(Keyword::Await),
            "break" => Token::Keyword(Keyword::Break),
            "case" => Token::Keyword(Keyword::Case),
            "catch" => Token::Keyword(Keyword::Catch),
            "class" => Token::Keyword(Keyword::Class),
            "const" => Token::Keyword(Keyword::Const),
            "continue" => Token::Keyword(Keyword::Continue),
            "debugger" => Token::Keyword(Keyword::Debugger),
            "default" => Token::Keyword(Keyword::Default),
            "delete" => Token::Keyword(Keyword::Delete),
            "do" => Token::Keyword(Keyword::Do),
            "else" => Token::Keyword(Keyword::Else),
            "export" => Token::Keyword(Keyword::Export),
            "extends" => Token::Keyword(Keyword::Extends),
            "false" => Token::Keyword(Keyword::False),
            "finally" => Token::Keyword(Keyword::Finally),
            "for" => Token::Keyword(Keyword::For),
            "function" => Token::Keyword(Keyword::Function),
            "if" => Token::Keyword(Keyword::If),
            "import" => Token::Keyword(Keyword::Import),
            "in" => Token::Keyword(Keyword::In),
            "instanceof" => Token::Keyword(Keyword::Instanceof),
            "new" => Token::Keyword(Keyword::New),
            "null" => Token::Keyword(Keyword::Null),
            "return" => Token::Keyword(Keyword::Return),
            "super" => Token::Keyword(Keyword::Super),
            "switch" => Token::Keyword(Keyword::Switch),
            "this" => Token::Keyword(Keyword::This),
            "throw" => Token::Keyword(Keyword::Throw),
            "true" => Token::Keyword(Keyword::True),
            "try" => Token::Keyword(Keyword::Try),
            "typeof" => Token::Keyword(Keyword::Typeof),
            "var" => Token::Keyword(Keyword::Var),
            "void" => Token::Keyword(Keyword::Void),
            "while" => Token::Keyword(Keyword::While),
            "with" => Token::Keyword(Keyword::With),
            _ => Token::Identifier(s)
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_numbers() {
        let mut lexer = Lexer::new("42");
        assert_eq!(lexer.next(), Some(Token::Number("42".to_string())));
    }
}