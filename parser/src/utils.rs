use common::{ast::{ASTBody, ASTNode, types::{ParenthesisKind, ParenthesisState, OperatorKind, TypeKind}}, errors::LangError};
use tokenizer::tokens::Token;
use crate::{errors::{PARAMETERS_EXPECTING_PARAMETER, ParsingErrorHelper, PARAMETERS_EXPECTING_COMMA, WRONG_TYPE}, parser::ParserScope};

#[macro_export]
macro_rules! expect_token {
    ($token: expr, $pattern: pat_param) => {
        let tok = $token;

        match tok {
            Some($pattern) => (),
            Some(_) => return Err(LangError::new_parser_unexpected_token()),
            None => return Err(LangError::new_parser_end_of_file()),
        }
    };
}

impl<'a> ParserScope<'a> {
    pub fn parse_object_values(&self, tokens: &mut Vec<Token>) -> Result<Vec<(String, ASTNode)>, LangError> {
        let mut res = Vec::new();
        let mut next_is_argument = true;
        
        loop {
            let token = tokens.pop();
                
            match token {
                Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close)) => break,
                Some(Token::Operator(OperatorKind::Comma)) => {
                    if next_is_argument {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_PARAMETER.to_string()));
                    } else {
                        next_is_argument = true;
                        continue;
                    }
                }
                Some(token) => {
                    if next_is_argument {
                        next_is_argument = false;

                        // name
                        let name = match token {
                            Token::Symbol(name) => name,
                            _ => return Err(LangError::new_parser_unexpected_token())
                        };
                        
                        // :
                        match tokens.pop() {
                            Some(Token::Operator(OperatorKind::Colon)) => (),
                            Some(_) => return Err(LangError::new_parser_unexpected_token()),
                            None => return Err(LangError::new_parser_end_of_file()),
                        }

                        // value
                        let value = self.parse_statement(tokens)?;
                        
                        res.push((name, value));
                    } else {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_COMMA.to_string()));
                    }
                },
                None => return Err(LangError::new_parser_end_of_file()),
            };
        }
        
        Ok(res)
    }

    /** Parses a block of tokens (something like "{ var x = 10; var y = false }").
     * It consumes only the last parenthesis and expectes the first token to be the first statement,
       in this case it will be "var"
     */ 
    pub fn parse_body(&self, tokens: &mut Vec<Token>) -> Result<ASTBody, LangError> {
        let mut body = Vec::new();
        
        loop {
            let token = tokens.last();
                
            let result = match token {
                Some(Token::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close)) => break,
                Some(_) => self.parse_statement(tokens)?,
                None => return Err(LangError::new_parser_end_of_file()),
            };
            
            body.push(result);
        }
        
        // Popping the last }
        tokens.pop();
        
        Ok(body)
    }

    /** Parses a list of parameter names (something like "(arg0, arg1, arg2)").
     * It consumes only the last parenthesis and expectes the first token to be the first argument,
       in this case it will be "arg0"
     */ 
    pub fn parse_parameter_names(&self, tokens: &mut Vec<Token>) -> Result<(Vec<String>, Vec<TypeKind>), LangError> {
        let mut names = Vec::new();
        let mut types = Vec::new();
        let mut next_is_argument = true;
        
        loop {
            let token = tokens.pop();
            
            match &token {
                Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close)) => break,
                Some(Token::Symbol(name)) => {
                    if next_is_argument {
                        next_is_argument = false;

                        let t = self.parse_type_error(tokens)?;

                        names.push(name.clone());
                        types.push(t);
                    } else {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_COMMA.to_string()));
                    }
                },
                Some(Token::Operator(OperatorKind::Comma)) => {
                    if next_is_argument {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_PARAMETER.to_string()));
                    } else {
                        next_is_argument = true;
                    }
                },
                Some(_) => return Err(LangError::new_parser_unexpected_token()),
                None => return Err(LangError::new_parser_end_of_file()),
            };
        }

        Ok((names, types))
    }
    
    pub fn parse_vector_values(&self, tokens: &mut Vec<Token>) -> Result<(TypeKind, ASTBody), LangError> {
        let mut body = Vec::new();
        let mut next_is_argument = true;
        let mut vector_type = TypeKind::Unknown;
        
        loop {
            let token = tokens.last();
                
            match token {
                Some(Token::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close)) => break,
                Some(Token::Operator(OperatorKind::Comma)) => {
                    if next_is_argument {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_PARAMETER.to_string()));
                    } else {
                        tokens.pop();

                        next_is_argument = true;
                        continue;
                    }
                }
                Some(_) => {
                    if next_is_argument {
                        next_is_argument = false;
                        
                        let node = self.parse_statement(tokens)?;
                        if vector_type.is_unknown() {
                            vector_type = node.eval_type.clone();
                        } else if vector_type != node.eval_type {
                            return Err(LangError::new_parser(WRONG_TYPE.to_string()));
                        }

                        body.push(node);
                    } else {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_COMMA.to_string()));
                    }
                },
                None => return Err(LangError::new_parser_end_of_file()),
            };
        }
        
        // Popping the last )
        tokens.pop();
        
        Ok((vector_type, body))
    }

    pub fn parse_parameter_values(&self, tokens: &mut Vec<Token>) -> Result<ASTBody, LangError> {
        let mut body = Vec::new();
        let mut next_is_argument = true;
        
        loop {
            let token = tokens.last();
                
            match token {
                Some(Token::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close)) => break,
                Some(Token::Operator(OperatorKind::Comma)) => {
                    if next_is_argument {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_PARAMETER.to_string()));
                    } else {
                        tokens.pop();

                        next_is_argument = true;
                        continue;
                    }
                }
                Some(_) => {
                    if next_is_argument {
                        next_is_argument = false;
                        body.push(self.parse_statement(tokens)?);
                    } else {
                        return Err(LangError::new_parser(PARAMETERS_EXPECTING_COMMA.to_string()));
                    }
                },
                None => return Err(LangError::new_parser_end_of_file()),
            };
        }
        
        // Popping the last )
        tokens.pop();
        
        Ok(body)
    }

    pub fn parse_type_option(&self, tokens: &mut Vec<Token>) -> Result<Option<TypeKind>, LangError> {
        // :
        match tokens.last() {
            Some(Token::Operator(OperatorKind::Colon)) => { tokens.pop(); },
            _ => return Ok(None)
        }

        // type
        match tokens.pop() {
            Some(Token::Type(tk)) => Ok(Some(tk)),
            _ => Err(LangError::new_parser_unexpected_token())
        }
    }

    pub fn parse_type_error(&self, tokens: &mut Vec<Token>) -> Result<TypeKind, LangError> {
        // :
        match tokens.last() {
            Some(Token::Operator(OperatorKind::Colon)) => { tokens.pop(); },
            _ => return Err(LangError::new_parser_unexpected_token())
        }

        // type
        match tokens.pop() {
            Some(Token::Type(tk)) => Ok(tk),
            _ => Err(LangError::new_parser_unexpected_token())
        }
    }
}