use common::tokens::TokenKind;

use super::resolver::{Resolver, AddResult};

pub struct WhitespaceResolver {
    previous_indentation: u32,
    indentation: Option<u32>,
}

impl WhitespaceResolver {
    pub fn new(current_indentation: u32) -> Self {
        Self {
            previous_indentation: current_indentation,
            indentation: None,
        }
    }
}

impl Resolver for WhitespaceResolver {
    fn add(&mut self, char: char) -> AddResult {
        let ret = match char {
            '\n' => {
                self.indentation = Some(0);

                AddResult::OkToken(TokenKind::NewLine)
            },
            '\t' => {
                if let Some(indentation) = self.indentation {
                    self.indentation = Some(indentation + 4)
                }

                AddResult::Ok
            },
            ' ' => {
                if let Some(indentation) = self.indentation {
                    self.indentation = Some(indentation + 1)
                }

                AddResult::Ok
            },
            c if c.is_whitespace() => AddResult::Ok,
            c => {
                // Checking for indentation changes
                if let Some(indentation) = self.indentation {
                    if indentation != self.previous_indentation {
                        return AddResult::ChangeIndentation(indentation, c)
                    }
                }

                AddResult::ChangeWithoutToken(c)
            },
        };

        ret
    }
}