use crate::{tokenizer::tokens::Token, common::lang_value::LangValue};
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
                AddResult::End(Token::Literal(LangValue::String(self.chars.clone())))
            }
        }
        
        self.add_char(char);

        AddResult::Ok
    }
}