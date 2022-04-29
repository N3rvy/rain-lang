use std::str::Chars;

use common::{errors::LangError, tokens::{Token, TokenKind}};

use crate::{resolvers::{resolver::{Resolver, AddResult}, whitespace_resolver::WhitespaceResolver}, iterator::Tokens};

pub struct Tokenizer<'a> {
    current_resolver: Box<dyn Resolver>,
    tokens: Vec<Token>,
    chars: Chars<'a>,
    last_token_pos: usize,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(source: &'a String) -> Result<Tokens, LangError> {
        let mut tokenizer = Self {
            current_resolver: Box::new(WhitespaceResolver::new()),
            tokens: Vec::new(),
            chars: source.chars(),
            last_token_pos: 0,
            pos: 0,
        };

        loop {
            let next_char = match tokenizer.next_char() {
                Some(c) => c,
                None => break,
            };

            tokenizer.tokenize_char(next_char)?;
        }

        tokenizer.tokenize_char('\n')?;
        
        Ok(Tokens::from_vec(tokenizer.tokens))
    }

    fn next_char(&mut self) -> Option<char> {
        self.pos += 1;
        self.chars.next()
    }

    fn push_token(&mut self, token: TokenKind) {
        self.tokens.push(Token::new(token, self.last_token_pos, self.pos));

        self.last_token_pos = self.pos;
    }

    fn tokenize_char(&mut self, char: char) -> Result<(), LangError> {
        let result = self.current_resolver.add(char);

        match result {
            AddResult::Ok => Ok(()),
            AddResult::OkToken(token) => {
                self.push_token(token);

                Ok(())
            },
            AddResult::End(token) => {
                self.push_token(token);

                let next_char = match self.next_char() {
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
            AddResult::ChangeChars(token, chars) => {
                self.push_token(token);
                
                self.current_resolver = self.resolver_from_char(chars[0]);
                
                for char in chars {
                    self.tokenize_char(char)?;
                }

                Ok(())
            }
            AddResult::Change(token, char) => {
                self.push_token(token);
                self.current_resolver = self.resolver_from_char(char);

                self.tokenize_char(char)
            },
            AddResult::Err(err) => Err(
                LangError::tokenizer(
                    self.tokens
                        .last()
                        .unwrap_or(&Token::new(TokenKind::NewLine, self.last_token_pos, self.pos)),
                    err)),
        }

    }
}