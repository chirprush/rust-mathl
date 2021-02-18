use super::token::Token;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            index: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let keywords = vec!["let", "if", "then", "else"];
        let whitespace = self.input[self.index..]
            .chars()
            .take_while(|c| c.is_ascii_whitespace())
            .collect::<String>()
            .len();
        self.index += whitespace;
        if self.index >= self.input.len() {
            return None;
        }
        match &self.input[self.index..self.index + 1] {
            "=" => {
                self.index += 1;
                Some(Ok(Token::Operator("=")))
            },
            "+" => {
                self.index += 1;
                Some(Ok(Token::Operator("+")))
            },
            "-" => {
                self.index += 1;
                Some(Ok(Token::Operator("-")))
            },
            "*" => {
                self.index += 1;
                Some(Ok(Token::Operator("*")))
            },
            "/" => {
                self.index += 1;
                Some(Ok(Token::Operator("/")))
            },
            ">" => {
                self.index += 1;
                Some(Ok(Token::Operator(">")))
            },
            "<" => {
                self.index += 1;
                Some(Ok(Token::Operator("<")))
            },
            "(" => {
                self.index += 1;
                Some(Ok(Token::Paren("(")))
            },
            ")" => {
                self.index += 1;
                Some(Ok(Token::Paren(")")))
            },
            _ => {
                let word = &self.input[self.index..]
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect::<String>();
                if word.len() == 0 {
                    Some(Err(format!("Unexpected character '{}' at column {}", &self.input[self.index..self.index+1], self.index + 1)))
                } else if word.chars().all(|c| c.is_ascii_digit()) {
                    self.index += word.len();
                    Some(Ok(Token::Int(word.parse::<i32>().unwrap())))
                } else if keywords.iter().any(|s| word.eq(s)) {
                    let result = Some(Ok(Token::Keyword(&self.input[self.index..self.index + word.len()])));
                    self.index += word.len();
                    result
                } else {
                    let result = Some(Ok(Token::Identifier(&self.input[self.index..self.index + word.len()])));
                    self.index += word.len();
                    result
                }
            }
        }
    }
}
