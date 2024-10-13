use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}

// need to use it as string
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


// TEMP
#[derive(Debug)]
pub enum Object {
    str_lit(String),
    flt_lit(f32),
}

// need to use it as string
impl fmt::Display for Object{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct Token {
  ttype: TokenType,
  lexeme: String,
  literal: Option<Object>,
  line: u32,
}

impl Token {

  pub fn token(ttype: TokenType, lexeme: String, literal: Option<Object>, line: u32) -> Self {
    Self {
    ttype,
    lexeme,
    literal,
    line,
    }
  }

}

// need to print
impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

