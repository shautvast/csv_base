use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug)]
pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    literal: Value,
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: impl Into<String>, literal: Value) -> Self {
        Self {
            tokentype,
            lexeme: lexeme.into(),
            literal,
        }
    }
}

pub(crate) fn add_keywords(keywords: &mut HashMap<String, TokenType>) {
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("NIL".to_string(), TokenType::Nil);
    keywords.insert("or".to_string(), TokenType::Or);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("select".to_string(), TokenType::Select);
    keywords.insert("from".to_string(), TokenType::From);
    keywords.insert("where".to_string(), TokenType::Where);
    keywords.insert("union".to_string(), TokenType::Union);
    keywords.insert("update".to_string(), TokenType::Update);
    keywords.insert("insert".to_string(), TokenType::Insert);
    keywords.insert("group".to_string(), TokenType::Group);
    keywords.insert("order".to_string(), TokenType::Order);
    keywords.insert("by".to_string(), TokenType::By);
    keywords.insert("having".to_string(), TokenType::Having);
    keywords.insert("sum".to_string(), TokenType::Sum);
    keywords.insert("max".to_string(), TokenType::Max);
    keywords.insert("min".to_string(), TokenType::Min);
    keywords.insert("delete".to_string(), TokenType::Delete);
    keywords.insert("commit".to_string(), TokenType::Commit);
    keywords.insert("describe".to_string(), TokenType::Describe);
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,
    Semicolon,
    Colon,
    Bang, // !
    Equals,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    BangEquals, // !=
    Unequal,    // <>
    Str,
    Num,
    Identifier,
    And,
    Else,
    False,
    Nil,
    Or,
    True,
    Select,
    From,
    Where,
    Union,
    Update,
    Insert,
    Group,
    Order,
    By,
    Having,
    Sum,
    Max,
    Min,
    Delete,
    Commit,
    Describe,
}
