use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    LAngle,
    RAngle,
    Plus,
    Minus,
    Dot,
    Comma,
    LBracket,
    RBracket,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            LAngle => write!(f, "<"),
            RAngle => write!(f, ">"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Dot => write!(f, "."),
            Comma => write!(f, ","),
            LBracket => write!(f, "["),
            RBracket => write!(f, "]"),
            EOF => write!(f, "<eof>"),
        }
    }
}
