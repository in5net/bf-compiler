use crate::Token;
use Token::*;

pub struct Lexer {
    text: String,
    index: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            index: 0,
            current_char: text.chars().nth(0).unwrap(),
            text,
        }
    }

    fn advance(&mut self) -> Token {
        self.index += 1;
        let next = self.text.chars().nth(self.index);
        self.current_char = match next {
            Some(c) => c,
            _ => '\0',
        };
        EOF
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut token = self.next_token();
        while token != EOF {
            tokens.push(token);
            token = self.next_token();
        }
        tokens.push(token);
        tokens
    }

    pub fn next_token(&mut self) -> Token {
        while self.current_char != '\0' {
            let token = match self.current_char {
                '<' => {
                    self.advance();
                    LAngle
                }
                '>' => {
                    self.advance();
                    RAngle
                }
                '+' => {
                    self.advance();
                    Plus
                }
                '-' => {
                    self.advance();
                    Minus
                }
                '.' => {
                    self.advance();
                    Dot
                }
                ',' => {
                    self.advance();
                    Comma
                }
                '[' => {
                    self.advance();
                    LBracket
                }
                ']' => {
                    self.advance();
                    RBracket
                }
                '\0' => EOF,
                _ => self.advance(),
            };
            if token != EOF {
                return token;
            }
        }
        EOF
    }
}
