use super::super::lexer::token::Token;
use super::node::Node;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser {
        Parser {
            tokens,
            index: 0
        }
    }

    pub fn status(&self) -> Result<(), String> {
        match self.index >= self.tokens.len() {
            true => Ok(()),
            _ => Err("Invalid syntax".to_string())
        }
    }

    fn peek(&self) -> Result<&Token<'a>, String> {
        if self.index >= self.tokens.len() {
            Err("Unexpected end of input".to_string())
        } else {
            Ok(&self.tokens[self.index])
        }
    }

    fn parse_number(&mut self) -> Result<Node, String> {
        match self.peek()? {
            Token::Int(n) => {
                // Although it might look like we can just increment self.index and then return
                // Ok(Node::Int(*n)), because of the borrow in self.peek, we actually can't.
                // Because of this, we have to assign it to result, increment self.index, and then
                // return result.
                let result = Ok(Node::Int(*n));
                self.index += 1;
                result
            },
            _ => Err("Expected number".to_string())
        }
    }

    fn parse_variable(&mut self) -> Result<Node, String> {
        match self.peek()? {
            Token::Identifier(ident) => {
                let result = Ok(Node::Identifier(ident.to_string()));
                self.index += 1;
                result
            }
            _ => Err("Expected an identifier".to_string())
        }
    }

    fn parse_paren(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        match self.peek()? {
            Token::Paren("(") => (),
            _ => return Err("Expected opening bracket in parenthesized expression".to_string())
        };
        self.index += 1;
        let expr = match self.parse_expr() {
            Ok(e) => e,
            error => {
                self.index = save_index;
                return error
            }
        };
        match self.peek() {
            Ok(Token::Paren(")")) => (),
            _ => {
                self.index = save_index;
                return Err("Expected closing bracket in parenthesized expression".to_string());
            }
        };
        self.index += 1;
        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        let mut result = self.parse_level1()?;
        loop {
            let op = match self.peek() {
                Ok(Token::Operator(">")) => ">",
                Ok(Token::Operator("<")) => "<",
                _ => break
            };
            self.index += 1;
            let right = match self.parse_level1() {
                Ok(result) => result,
                error => {
                    self.index = save_index;
                    return error;
                },
            };
            result = Node::BinaryOp(Box::new(result), op.to_string(), Box::new(right));
        }
        Ok(result)
    }

    fn parse_level1(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        let mut result = self.parse_level2()?;
        loop {
            let op = match self.peek() {
                Ok(Token::Operator("+")) => "+",
                Ok(Token::Operator("-")) => "-",
                _ => break
            };
            self.index += 1;
            let right = match self.parse_level2() {
                Ok(result) => result,
                error => {
                    self.index = save_index;
                    return error;
                },
            };
            result = Node::BinaryOp(Box::new(result), op.to_string(), Box::new(right));
        }
        Ok(result)
    }

    fn parse_level2(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        let mut result = self.parse_factor()?;
        loop {
            let op = match self.peek() {
                Ok(Token::Operator("*")) => "*",
                Ok(Token::Operator("/")) => "/",
                _ => break
            };
            self.index += 1;
            let right = match self.parse_factor() {
                Ok(result) => result,
                error => {
                    self.index = save_index;
                    return error;
                },
            };
            result = Node::BinaryOp(Box::new(result), op.to_string(), Box::new(right));
        }
        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<Node, String> {
        match self.peek()? {
            Token::Paren("(") => self.parse_paren(),
            Token::Keyword("if") => self.parse_if(),
            Token::Operator("-") => self.parse_unary(),
            Token::Int(_) => self.parse_number(),
            Token::Identifier(_) => self.parse_variable(),
            _ => Err("Failed to parse value".to_string())
        }
    }

    fn parse_unary(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        let op = match self.peek()? {
            Token::Operator("-") => "-",
            _ => return Err("Expected a unary operator".to_string())
        };
        self.index += 1;
        let expr = match self.parse_factor() {
            Ok(expr) => expr,
            error => {
                self.index = save_index;
                return error;
            }
        };
        Ok(Node::UnaryOp(op.to_string(), Box::new(expr)))
    }

    fn parse_if(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        match self.peek()? {
            Token::Keyword("if") => (),
            _ => return Err("Expected 'if' keyword in binding statement".to_string())
        };
        self.index += 1;
        let case = match self.parse_expr() {
            Ok(expr) => expr,
            error => {
                self.index = save_index;
                return error;
            }
        };
        match self.peek() {
            Ok(Token::Keyword("then")) => (),
            _ => {
                self.index = save_index;
                return Err("Expected keyword 'then' after case expression in if statement".to_string());
            }
        };
        self.index += 1;
        let left = match self.parse_expr() {
            Ok(expr) => expr,
            error => {
                self.index = save_index;
                return error;
            }
        };
        match self.peek() {
            Ok(Token::Keyword("else")) => (),
            _ => {
                self.index = save_index;
                return Err("Expected keyword 'else' after left expression in if statement".to_string());
            }
        };
        self.index += 1;
        let right = match self.parse_expr() {
            Ok(expr) => expr,
            error => {
                self.index = save_index;
                return error;
            }
        };
        Ok(Node::If(Box::new(case), Box::new(left), Box::new(right)))
    }

    fn parse_let(&mut self) -> Result<Node, String> {
        let save_index = self.index;
        match self.peek()? {
            Token::Keyword("let") => (),
            _ => return Err("Expected 'let' keyword in binding statement".to_string())
        };
        self.index += 1;
        let ident = match self.peek() {
            Ok(Token::Identifier(ident)) => *ident,
            _ => {
                self.index = save_index;
                return Err("Expected an identifier after 'let' keyword".to_string())
            }
        };
        self.index += 1;
        match self.peek() {
            Ok(Token::Operator("=")) => (),
            _ => {
                self.index = save_index;
                return Err("Expected an equals sign in binding statement".to_string())
            }
        };
        self.index += 1;
        let expr = match self.parse_expr() {
            Ok(expr) => expr,
            error => {
                self.index = save_index;
                return error;
            }
        };
        self.index += 1;
        Ok(Node::Let(ident.to_string(), Box::new(expr)))
    }

    pub fn parse_all(&mut self) -> Result<Node, String> {
        match self.peek()? {
            Token::Keyword("let") => self.parse_let(),
            _ => self.parse_expr()
        }
    }
}
