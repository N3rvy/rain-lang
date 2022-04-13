use common::{errors::TokenizerErrorKind, ast::types::LiteralKind, tokens::TokenKind};
use super::resolver::{Resolver, AddResult};

pub struct StringResolver {
    chars: String,
}

impl StringResolver {
    pub fn new() -> Self {
        Self {
            chars: String::new(),
        }
    }

    fn parse_string(string: &String) -> Result<String, TokenizerErrorKind> {
        let backslashes = Self::count_special_caracters(string);
        let mut res = Vec::<u8>::with_capacity(string.len() - backslashes);
        
        let mut next_is_special = false;
        
        for c in string.chars() {
            if next_is_special {
                res.push(match c {
                    'n' => '\n',
                    'r' => 'r',
                    't' => '\t',
                    c => c,
                } as u8);
                next_is_special = false;
                continue;
            }

            if c == '\\' {
                next_is_special = true;
                continue;
            }

            res.push(c as u8);
        }
        
        match String::from_utf8(res) {
            Ok(val) => Ok(val),
            Err(_) => Err(TokenizerErrorKind::InvalidStringLiteral),
        }
    }

    /** Counts the special characters such as '\n' that occur inside a string */
    fn count_special_caracters(string: &String) -> usize {
        let mut res = 0;
        // This is used for skipping double backslashes '\\' from counting
        let mut last_was_backslash = false;
        
        for c in string.chars() {
            if c == '\\' && !last_was_backslash {
                res += 1;
                last_was_backslash = true;
            } else {
                last_was_backslash = false;
            }
        }
        
        res
    }
}

impl Resolver for StringResolver {
    fn add(&mut self, char: char) -> AddResult {
        if char == '"' {
            return if self.chars.len() == 0 {
                AddResult::Ok
            } else {
                match Self::parse_string(&self.chars) {
                    Ok(value) => AddResult::End(TokenKind::Literal(LiteralKind::String(value))),
                    Err(err) => AddResult::Err(err),
                }
            }
        }
        
        self.chars.push(char);

        AddResult::Ok
    }
}