use crate::error::LangError;

use super::{resolvers::resolver::{Resolver, ResolverKind, AddResult}, tokens::Token};


pub fn tokenize(mut script: String) -> Result<Vec<Token>, LangError> {
    let mut tokens = Vec::new();
    
    let mut resolver: Resolver = Resolver::new_empty();
    
    script.push('\n');
    
    for char in script.chars() {
        if matches!(resolver.kind, ResolverKind::None) {
            resolver = match char {
                c if c.is_whitespace() => continue,
                '0'..='9' => Resolver::new_number(),
                '=' | '!' | '>' | '<' | '+' | '-' | '*' | '/' | '%' | '^' => Resolver::new_operator(),
                '"' => Resolver::new_string_literal(),
                _ => Resolver::new_symbol(),
            }
        }
        
        let result = resolver.add(char);
        
        match result {
            AddResult::Ok => (),
            AddResult::End(token) => {
                tokens.push(token);
                resolver = Resolver::new_empty();
            },
            AddResult::Err(err) => return Err(err),
        }
    }
    
    Ok(tokens)
}