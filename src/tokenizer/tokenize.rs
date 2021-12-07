use crate::error::LangError;

use super::{resolvers::resolver::{Resolver, ResolverKind, AddResult}, tokens::Token};


pub fn tokenize(mut script: String) -> Result<Vec<Token>, LangError> {
    let mut tokens = Vec::new();
    
    let mut resolver: Resolver = Resolver::new_empty();
    
    script.push('\n');
    
    for char in script.chars() {
        if matches!(resolver.kind, ResolverKind::None) {
            resolver = match Resolver::from_char(char) {
                Some(res) => res,
                None => continue,
            }
        }
        
        let result = resolver.add(char);
        
        match result {
            AddResult::Ok => (),
            AddResult::End(token) => {
                tokens.push(token);
                resolver = Resolver::new_empty();
            },
            AddResult::Changed(token, res) => {
                tokens.push(token);
                resolver = res;
            },
            AddResult::Err(err) => return Err(err),
        }
    }
    
    Ok(tokens)
}