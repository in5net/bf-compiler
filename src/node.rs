use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Left,
    Right,
    Inc,
    Dec,
    Output,
    Input,
    Group(Vec<Node>),
    EOF,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Node::*;
        match self {
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            Inc => write!(f, "+"),
            Dec => write!(f, "-"),
            Output => write!(f, "."),
            Input => write!(f, ","),
            Group(nodes) => write!(
                f,
                "([{}])",
                nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            EOF => write!(f, "<eof>"),
        }
    }
}
