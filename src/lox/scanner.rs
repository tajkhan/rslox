use std::collections::HashMap;
//mod token;
use super::token::{TokenType::{self, *}, Token, Object};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,

    // location trackers
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new (source: String) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
            keywords: Self::init_keywords(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn init_keywords() -> HashMap<String, TokenType> {
        let mut kw = HashMap::new();
        kw.insert("and".to_string(),    AND);
        kw.insert("class".to_string(),  CLASS);
        kw.insert("else".to_string(),   ELSE);
        kw.insert("false".to_string(),  FALSE);
        kw.insert("for".to_string(),    FOR);
        kw.insert("fun".to_string(),    FUN);
        kw.insert("if".to_string(),     IF);
        kw.insert("nil".to_string(),    NIL);
        kw.insert("or".to_string(),     OR);
        kw.insert("print".to_string(),  PRINT);
        kw.insert("return".to_string(), RETURN);
        kw.insert("super".to_string(),  SUPER);
        kw.insert("this".to_string(),   THIS);
        kw.insert("true".to_string(),   TRUE);
        kw.insert("var".to_string(),    VAR);
        kw.insert("while".to_string(),  WHILE);
        kw
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::token(TokenType::EOF, "".to_string(), None, self.line));
        return std::mem::take(&mut self.tokens);
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // 1 char operators
          '(' => self.add_token_1(LEFT_PAREN),
          ')' => self.add_token_1(RIGHT_PAREN),
          '{' => self.add_token_1(LEFT_BRACE),
          '}' => self.add_token_1(RIGHT_BRACE),
          ',' => self.add_token_1(COMMA),
          '.' => self.add_token_1(DOT),
          '-' => self.add_token_1(MINUS),
          '+' => self.add_token_1(PLUS),
          ';' => self.add_token_1(SEMICOLON),
          '*' => self.add_token_1(STAR), 

            // 2 char operators
          '!' => { let op = if self.mmatch('=') { BANG_EQUAL } else {BANG};
                   self.add_token_1(op); },
          '=' => { let op = if self.mmatch('=') { EQUAL_EQUAL } else {EQUAL};
                   self.add_token_1(op); },
          '<' => { let op = if self.mmatch('=') { LESS_EQUAL } else {  LESS};
                   self.add_token_1(op); },
          '>' => { let op = if self.mmatch('=') { GREATER_EQUAL } else {GREATER};
                   self.add_token_1(op); },

            // '/' or comment?
          '/' => {
            if self.mmatch('/') {
                // A comment goes until the end of the line.
                while self.peek() != '\n' && !self.is_end() {self.advance();}
            } else {
                self.add_token_1(SLASH);
            }
          },

            // white space
            ' ' | '\r' | '\t' => (),    // ignore
            '\n' => {self.line += 1;},

            // string literals
            '"'=> self.string(),

            // numeric literals

            // default
           _  => if Self::is_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    println!("Error line {0}: Unexpected character.", self.line); // reconcile with lox impl!
                },
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let id = &self.source[self.start .. self.current];
        if let Some(ttype) = self.keywords.get(id) {
            self.add_token_1(*ttype);
        } else {
            self.add_token_1(IDENTIFIER);
        }
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek())  {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let flt = self.source[self.start .. self.current].parse::<f32>().unwrap();
        self.add_token_2(NUMBER, Some(Object::flt_lit(flt)));
    }



    fn advance(&mut self) -> char {
        // let c = self.source.as_bytes()[self.current];    // assumes ASCII
        let c = self.source.chars().nth(self.current).unwrap(); // error handling?
        self.current += 1;
        c
    }

    fn add_token_1(&mut self, ttype: TokenType) {
        self.add_token_2(ttype, None);
    }

    // overloading ???
    fn add_token_2(&mut self, ttype: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current]; // TODO: & here: str size not known comp time??
        self.tokens.push(Token::token(ttype, text.to_string(), literal, self.line));
    }

    fn mmatch(&mut self, expected: char) -> bool {
        if self.is_end() {return false};
        if self.source.chars().nth(self.current).unwrap() != expected {return false};

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_end() {return '\0'};
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source.chars().nth(self.current + 1).unwrap()  // error handling??
    }

    fn is_alpha (c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn is_digit (c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
          if self.peek() == '\n' { self.line += 1; }
          self.advance();
        }

        if self.is_end() {
            println!("Error line {0}: Unexpected character.", self.line); // reconcile with lox impl!
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1 .. self.current - 1];  // error without & ??
        self.add_token_2(STRING, Some(Object::str_lit(value.to_string())));
    }
}
