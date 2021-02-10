#[derive(Debug)]
pub enum Token<'a> {
    Keyword(&'a str),
    Identifier(&'a str),
    Int(i32),
    Operator(&'a str),
    Paren(&'a str),
}
