use std::sync::Arc;
use common::tokens::{Token, TokenSnapshot};

#[derive(Debug, Clone)]
pub struct Tokens {
    tokens: Arc<Vec<Token>>,
    current_pos: TokenSnapshot,
}

impl Tokens {
    pub fn from_vec(vec: Vec<Token>) -> Self {
        Self {
            tokens: Arc::new(vec),
            current_pos: TokenSnapshot(0),
        }
    }

    pub fn new_clone(&self, snapshot: TokenSnapshot) -> Tokens {
        Self {
            tokens: self.tokens.clone(),
            current_pos: snapshot,
        }
    }
    
    pub fn pop(&mut self) -> Option<Token> {
        let pos = self.current_pos.0;

        // Adding it after
        self.current_pos.0 += 1;

        if pos >= self.tokens.len() {
            return None
        }
        Some(self.tokens[pos].clone())
    }
    
    pub fn peek(&self) -> Option<Token> {
        if self.current_pos.0 >= self.tokens.len() {
            return None
        }

        Some(self.tokens[self.current_pos.0].clone())
    }
    
    pub fn has_next(&self) -> bool {
        self.current_pos.0 < self.tokens.len()
    }
    
    pub fn snapshot(&self) -> TokenSnapshot {
        self.current_pos
    }
    
    pub fn rollback(&mut self, snapshot: TokenSnapshot) {
        self.current_pos = snapshot
    }
}