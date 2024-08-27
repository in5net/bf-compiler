use crate::{Node, Token};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            token: tokens[0].clone(),
            tokens,
            index: 0,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        let next = self.tokens.get(self.index);
        self.token = match next {
            Some(token) => token.clone(),
            _ => Token::EOF,
        };
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        use Token::*;
        while self.token != EOF {
            let node = self.expr();
            nodes.push(node);
        }
        nodes
    }

    fn expr(&mut self) -> Node {
        use Token::*;
        match self.token {
            LAngle => {
                self.advance();
                Node::Left
            }
            RAngle => {
                self.advance();
                Node::Right
            }
            Plus => {
                self.advance();
                Node::Inc
            }
            Minus => {
                self.advance();
                Node::Dec
            }
            Dot => {
                self.advance();
                Node::Output
            }
            Comma => {
                self.advance();
                Node::Input
            }
            LBracket => self.group(),
            _ => panic!("Unexpected token: {}", self.token),
        }
    }

    fn group(&mut self) -> Node {
        if self.token != Token::LBracket {
            panic!("{}", "expected '['");
        }
        self.advance();

        let mut nodes = Vec::new();
        while self.token != Token::RBracket {
            let node = self.expr();
            nodes.push(node);
        }

        if self.token != Token::RBracket {
            panic!("{}", "expected ']'");
        }
        self.advance();

        Node::Group(nodes)
    }
}
