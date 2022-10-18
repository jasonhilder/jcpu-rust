use regex::Regex;

use crate::structures::{Token, TokenType};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
    line: usize,
    pos: usize,
    input: String,
    tmp_string: String,
    pub tokens: Vec<Token>,
    pub labels: HashMap<String, usize>
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            line: 0,
            pos: 0,
            input: input.to_string(),
            tokens: vec![],
            tmp_string: String::new(),
            labels: HashMap::new()
        }
    }

    pub fn parse(&mut self) {
        let mut bites = self.input.chars().peekable();

        while let Some(bite) = bites.next() {
            // increment character pos
            self.pos += 1;
            match bite {
                ',' | ' ' | '\n' | '\r' | '\t' => {
                    if !self.tmp_string.is_empty() {
                        self.tokens.push(self.create_token());

                        // reset tmp string
                        self.tmp_string = String::new();

                        // account for pos and line
                        if bite == '\n' || bite == '\r' {
                            self.line += 1;
                            self.pos = 0;
                        }

                        // account for commas
                        if bite == ',' {
                            self.tokens.push(Token{
                                ttype: TokenType::Comma,
                                tvalue: String::from(bite),
                                line: self.line,
                                column: self.pos,
                            })
                        }
                    }
                },
                ':' => { // labels dont get compiled into instructions so we just track the current line
                    if !self.tmp_string.is_empty() {
                        self.tokens.push(Token {
                            ttype: TokenType::LabelSrc,
                            tvalue: self.tmp_string.clone(),
                            line: self.line,
                            column: self.pos
                        });

                        // reset tmp string
                        self.tmp_string = String::new();

                        self.line += 1;
                        self.pos = 0;
                    }
                },
                '$' => {
                    self.tokens.push(Token{
                        ttype: TokenType::LabelDst,
                        tvalue: self.tmp_string.clone(),
                        line: self.line,
                        column: self.pos,
                    });
                },
                ';' => {
                    while let Some(c) = bites.peek() {
                        if *c == '\r' {
                            bites.next();
                            break;
                        }
                        if *c == '\n' {
                            break;
                        }

                        bites.next(); // just keep skipping
                    }
                },
                _ => self.tmp_string.push(bite)

            }

        }

    }

    fn create_token(&self) -> Token {
        if self.is_string() {
            return Token{
                ttype: TokenType::Identifier,
                tvalue: self.tmp_string.clone(),
                line: self.line,
                column: self.pos,
            }
        }

        if self.is_number() {
            return Token{
                ttype: TokenType::Value,
                tvalue: self.tmp_string.clone(),
                line: self.line,
                column: self.pos,
            }
        }

        if self.is_hex() {
            return Token{
                ttype: TokenType::Value,
                tvalue: self.hex_to_dec(),
                line: self.line,
                column: self.pos,
            }
        }

        panic!("Error parsing on line: {}, column: {}", self.line, self.pos - 1)
    }

    fn is_number(&self) -> bool {
        self.tmp_string.parse::<u8>().is_ok()
    }

    fn is_string(&self) -> bool {
        let str_regex = Regex::new(r"^[a-zA-Z]+[a-zA-Z0-9]*$").unwrap();

        str_regex.is_match(self.tmp_string.as_str())
    }

    fn is_hex(&self) -> bool {
        self.tmp_string.contains("0x")
    }

    fn hex_to_dec(&self) -> String {
        let x = self.tmp_string.clone();
        let without_prefix = x.trim_start_matches("0x");
        let z = u8::from_str_radix(without_prefix, 16);

        if let Ok(hex_num) = z {
            hex_num.to_string()
        } else {
            panic!("Error parsing hex value on line: {}, character: {}", self.line, self.pos - 1)
        }
    }
}
