use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    Error(String),
    Int(i32),
    Identifier(String),
    Operation(Box<Node>, String, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Let(String, Box<Node>),
}

impl Node {
    pub fn is_error(&self) -> bool {
        if let Self::Error(_) = self {
            true
        } else {
            false
        }
    }

    pub fn eval(&self, mut env: &mut HashMap<String, Node>) -> Self {
        match self {
            Self::Error(message) => Self::Error(message.to_string()),
            Self::Int(n) => Self::Int(*n),
            Self::Identifier(ident) => {
                match env.get(&ident.to_string()) {
                    Some(node) => node.clone(),
                    None => Self::Error(format!("Variable '{}' does not exist", ident))
                }
            },
            Self::Operation(left, op, right) => {
                let left = (*left).eval(&mut env);
                let right = (*right).eval(&mut env);
                match &op[..] {
                    "+" => left.add(&right),
                    "-" => left.minus(&right),
                    "*" => left.mult(&right),
                    "/" => left.div(&right),
                    _ => panic!(format!("Operator {} not yet implemented", op))
                }
            }
            Self::If(case, left, right) => {
                let case = (*case).eval(&mut env);
                if case.is_error() {
                    return case;
                }
                match case {
                    Self::Int(0) => (*right).eval(&mut env),
                    Self::Int(_) => (*left).eval(&mut env),
                    _ => Self::Error("Case value cannot be tested in an if statement".to_string())
                }
            }
            Self::Let(ident, expr) => {
                let value = (*expr).eval(&mut env);
                if value.is_error() {
                    return value;
                }
                env.insert(ident.to_string(), value.clone());
                value
            }
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match self {
            Self::Int(left) => match other {
                Self::Int(right) => Self::Int(left + right),
                Self::Error(message) => Self::Error(message.to_string()),
                _ => Self::Error("Cannot add a non-integer value".to_string())
            }
            Self::Error(message) => Self::Error(message.to_string()),
            _ => Self::Error("Cannot add a non-integer value".to_string())
        }
    }

    pub fn minus(&self, other: &Self) -> Self {
        match self {
            Self::Int(left) => match other {
                Self::Int(right) => Self::Int(left - right),
                Self::Error(message) => Self::Error(message.to_string()),
                _ => Self::Error("Cannot subtract a non-integer value".to_string())
            },
            Self::Error(message) => Self::Error(message.to_string()),
            _ => Self::Error("Cannot subtract a non-integer value".to_string())
        }
    }

    pub fn mult(&self, other: &Self) -> Self {
        match self {
            Self::Int(left) => match other {
                Self::Int(right) => Self::Int(left * right),
                Self::Error(message) => Self::Error(message.to_string()),
                _ => Self::Error("Cannot multiply a non-integer value".to_string())
            },
            Self::Error(message) => Self::Error(message.to_string()),
            _ => Self::Error("Cannot multiply a non-integer value".to_string())
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        match self {
            Self::Int(left) => match other {
                Self::Int(0) => Self::Error("Cannot divide by zero".to_string()),
                Self::Int(right) => Self::Int(left / right),
                Self::Error(message) => Self::Error(message.to_string()),
                _ => Self::Error("Cannot multiply a non-integer value".to_string())
            },
            Self::Error(message) => Self::Error(message.to_string()),
            _ => Self::Error("Cannot multiply a non-integer value".to_string())
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error(message) => write!(f, "\x1b[31m{}\x1b[0m", message),
            Self::Int(n) => write!(f, "\x1b[32m{}\x1b[0m", n),
            Self::Identifier(_) => panic!("Cannot display node of type Identifier"),
            Self::Operation(_, _, _) => panic!("Cannot display node of type Operation"),
            Self::If(_, _, _) => panic!("Cannot display node of type If"),
            Self::Let(_, _) => panic!("Cannot display node of type Let"),
        }
    }
}
