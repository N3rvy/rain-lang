use common::{lang_value::LangValue, errors::LangError, messages::INVALID_STRING_LITERAL};

use crate::tokens::Token;

use super::resolver::{Resolver, ResolverKind, AddResult};

impl Resolver {
    pub(crate) fn new_string_literal() -> Self {
        Self {
            kind: ResolverKind::StringLiteral,
            add_fn: Self::add_string_literal,
            chars: Default::default(),
        }
    }
    
    fn add_string_literal(&mut self, char: char) -> AddResult {
        if char == '"' {
            return if self.chars.len() == 0 {
                AddResult::Ok
            } else {
                match parse_string(&self.chars) {
                    Ok(value) => AddResult::End(Token::Literal(LangValue::String(value))),
                    Err(err) => AddResult::Err(err),
                }
            }
        }
        
        self.add_char(char);

        AddResult::Ok
    }
}

fn parse_string(string: &String) -> Result<String, LangError> {
    let backslashes = count_special_caracters(string);
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
        Err(_) => Err(LangError::new_tokenizer(INVALID_STRING_LITERAL.to_string())),
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