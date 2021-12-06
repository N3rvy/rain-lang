use super::{resolvers::resolver::{Resolver, ResolverKind}, tokens::Token};


pub fn tokenize(script: String) -> Vec<Token> {
    let tokens = Vec::new();
    
    let mut char_buffer = Vec::new();
    let mut resolver = Resolver::new();
    
    for char in script.chars() {
        if matches!(resolver.kind, ResolverKind::StringLiteral) {
            resolver.add(char, &mut char_buffer);
        }

        match char {
        }
    }
    
    tokens
}