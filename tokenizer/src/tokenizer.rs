use std::str::Chars;

use common::errors::LangError;

use crate::{tokens::Token, resolvers::{resolver::{Resolver, AddResult}, whitespace_resolver::WhitespaceResolver}, iterator::Tokens, errors::INVALID_INDENT};

pub struct Tokenizer<'a> {
    current_resolver: Box<dyn Resolver>,
    tokens: Vec<Token>,
    chars: Chars<'a>,
    pub(crate) indentation_stack: Vec<u32>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(source: &'a String) -> Result<Tokens, LangError> {
        let mut tokenizer = Self {
            current_resolver: Box::new(WhitespaceResolver::new(0)), // TODO: Maybe there is a preattier way (without the maybe but i am too lazy to do it right now)
            tokens: Vec::new(),
            chars: source.chars(),
            indentation_stack: vec![0],
        };
        loop {
            let next_char = match tokenizer.chars.next() {
                Some(c) => c,
                None => break,
            };

            tokenizer.tokenize_char(next_char)?;
        }

        tokenizer.tokenize_char('\n')?;
        
        Ok(Tokens::from_vec(tokenizer.tokens))
    }

    fn tokenize_char(&mut self, char: char) -> Result<(), LangError> {
        let result = self.current_resolver.add(char);

        match result {
            AddResult::Ok => Ok(()),
            AddResult::End(token) => {
                self.tokens.push(token);

                let next_char = match self.chars.next() {
                    Some(c) => c,
                    None => return Ok(()),
                };

                self.current_resolver = self.resolver_from_char(next_char);

                self.tokenize_char(next_char)
            },
            AddResult::ChangeWithoutToken(char) => {
                self.current_resolver = self.resolver_from_char(char);

                self.tokenize_char(char)
            },
            AddResult::ChangeIndentation(new_indent, char) => {
                if new_indent < self.indentation_stack.last().unwrap().clone() {
                    // In case multiple dedentation occur in a single one

                    // Already popping one
                    self.indentation_stack.pop();

                    loop {
                        self.tokens.push(Token::Dedent);

                        match self.indentation_stack.last() {
                            Some(indent) => {
                                // If we are now at the same indentation level simply exit the loop
                                if indent.clone() == new_indent {
                                    break
                                }

                                // Otherwise pop the indentaion from the stack
                                self.indentation_stack.pop();
                            },
                            None => return Err(LangError::new_tokenizer(INVALID_INDENT.to_string()))    ,
                        };

                    }
                } else {
                    self.tokens.push(Token::Indent);
                    self.indentation_stack.push(new_indent);
                }

                self.current_resolver = self.resolver_from_char(char);
                self.tokenize_char(char)
            },
            AddResult::ChangeChars(token, chars) => {
                self.tokens.push(token);
                
                self.current_resolver = self.resolver_from_char(chars[0]);
                
                for char in chars {
                    self.tokenize_char(char)?;
                }

                Ok(())
            }
            AddResult::Change(token, char) => {
                self.tokens.push(token);
                self.current_resolver = self.resolver_from_char(char);

                self.tokenize_char(char)
            },
            AddResult::Err(err) => Err(err),
        }

    }
}