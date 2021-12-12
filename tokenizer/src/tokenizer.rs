use common::errors::LangError;

use crate::{tokens::Token, resolvers::resolver::{Resolver, ResolverKind, AddResult}};


pub fn tokenize(mut script: String) -> Result<Vec<Token>, LangError> {
    let mut tokens = Vec::new();
    
    let mut resolver: Resolver = Resolver::new_empty();
    
    script.push('\n');
    
    for char in script.chars() {
        if matches!(resolver.kind, ResolverKind::None) {
            if char.is_whitespace() {
                continue;
            }

            resolver = Resolver::from_char(char);
        }
        
        let result = resolver.add(char);
        
        let hr_result = handle_result(result, &mut tokens, &mut resolver);
        
        match hr_result {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }
    
    Ok(tokens)
}

fn handle_result(result: AddResult, tokens: &mut Vec<Token>, resolver: &mut Resolver) -> Result<(), LangError> {
   match result {
        AddResult::Ok => Ok(()),
        AddResult::End(token) => {
            tokens.push(token);
            *resolver = Resolver::new_empty();
            Ok(())
        },
        AddResult::ChangeChars(token, chars) => {
            println!("token: {:?}, chars: {:?}", token, chars);
            tokens.push(token);
            
            *resolver = Resolver::from_char(chars[0]);
            
            for i in 0..chars.len() {
                let result = resolver.add(chars[i]);
                handle_result(result, tokens, resolver)?;
            }
            
            Ok(())
        }
        AddResult::Change(token, char) => {
            tokens.push(token);
            
            *resolver = Resolver::from_char(char);
            
            match resolver.kind {
                ResolverKind::None => Ok(()),
                _ => {
                    let result = resolver.add(char);
                    handle_result(result, tokens, resolver)
                },
            }
        },
        AddResult::Err(err) => Err(err),
    }
}