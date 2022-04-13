use common::{errors::{TokenizerErrorKind}, ast::types::LiteralKind, tokens::TokenKind};
use super::resolver::{Resolver, AddResult};


pub struct NumberResolver {
    chars: String,
}

impl NumberResolver {
    pub fn new() -> Self {
        Self {
            chars: String::new(),
        }
    }
    
    fn end_number(&self) -> Result<TokenKind, TokenizerErrorKind>  {
        if self.chars.contains('.') {
            match self.chars.parse::<f32>() {
                Ok(value) => Ok(TokenKind::Literal(LiteralKind::Float(value))),
                Err(_) => Err(TokenizerErrorKind::FloatParse(self.chars.clone())),
            }
        } else {
            match self.chars.parse::<i32>() {
                Ok(value) => Ok(TokenKind::Literal(LiteralKind::Int(value))),
                Err(_) => Err(TokenizerErrorKind::IntParse(self.chars.clone())),
            }
        }
    }
}

impl Resolver for NumberResolver {
    fn add(&mut self, char: char) -> AddResult {
        match char {
            '0'..='9' => {
                self.chars.push(char);
                AddResult::Ok
            },
            '.' => {
                // If there is a second point then switch this resolver from a number resolver to an operator resolver
                if self.chars.contains('.') {
                    if self.chars.chars().last().unwrap() == '.' {
                        // Removing the last "."
                        self.chars.remove(self.chars.len() - 1);
                        
                        match self.end_number() {
                            Ok(token) => AddResult::ChangeChars(token, vec!['.', '.']),
                            Err(err) => AddResult::Err(err),
                        }
                    } else {
                        match self.end_number() {
                            Ok(token) => AddResult::Change(token, char),
                            Err(err) => AddResult::Err(err),
                        }
                    }
                } else {
                    self.chars.push(char);
                    AddResult::Ok
                }
            },

            _ => {
                match self.end_number() {
                    Ok(token) => AddResult::Change(token, char),
                    Err(err) => AddResult::Err(err),
                }
            }
        }
    }
}