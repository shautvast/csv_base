use std::collections::HashMap;

use anyhow::anyhow;

use crate::value::Value;

use super::tokens::{Token, TokenType};

pub fn parse(sql: &str) -> anyhow::Result<Vec<Token>> {
    let mut scanner = Scanner::new(sql);
    scanner.scan_tokens()?;
    Ok(scanner.tokens)
}

struct Scanner {
    source: String,
    source_chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    fn new(sql: &str) -> Self {
        let mut new = Self {
            source: sql.to_string(),
            source_chars: sql.to_string().chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            keywords: HashMap::new(),
        };

        crate::sql::tokens::add_keywords(&mut new.keywords);
        new
    }

    fn scan_tokens(&mut self) -> anyhow::Result<()> {
        self.start = self.current;
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(())
    }

    fn scan_token(&mut self) -> anyhow::Result<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => {
                if self.match_token('-') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Minus);
                }
            }
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '<' => {
                let token = if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token)
            }
            '>' => {
                let token = if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token)
            }
            ' ' | '\t' | '\r' | '\n' => {}
            '\'' => self.string()?,
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(anyhow!("Unexpected character '{}'", c));
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let tokentype = self.keywords.get(&text.to_lowercase());

        self.add_token(if let Some(tokentype) = tokentype {
            *tokentype
        } else {
            TokenType::Identifier
        });
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if (self.peek() == '.' || self.peek() == ',') && is_digit(self.peek_next()) {
            self.advance();
        }

        self.add_literal(TokenType::Num, self.source[self.start..self.current].into());
    }

    fn string(&mut self) -> anyhow::Result<()> {
        while self.peek() != '\'' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(anyhow!("Unterminated string value"));
        }

        self.advance();

        let string = self.source[self.start + 1..self.current - 1].to_string();
        self.add_literal(TokenType::Str, string.into());
        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source_chars.len() {
            '\0'
        } else {
            self.source_chars[self.current + 1]
        }
    }

    fn add_token(&mut self, tokentype: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(tokentype, text, Value::null()));
    }

    fn add_literal(&mut self, tokentype: TokenType, literal: Value) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(tokentype, text, literal));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_chars[self.current - 1]
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source_chars[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_chars.len()
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = parse("select name from employee;");
        println!("{:?}", tokens);
    }
}
