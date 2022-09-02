
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Comma,
    Value
}

#[derive(Debug,Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub tvalue: String,
    pub line: usize,
    pub column: usize,
}
