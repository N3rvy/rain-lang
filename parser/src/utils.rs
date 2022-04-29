use common::{ast::{ASTBody, ASTNode, types::{ParenthesisKind, ParenthesisState, OperatorKind, TypeKind, MathOperatorKind}}, errors::{LangError, ParserErrorKind}, tokens::{Token, TokenKind}};
use tokenizer::iterator::Tokens;
use crate::{errors::ParsingErrorHelper, parser_scope::ParserScope};

#[macro_export]
macro_rules! expect_token {
    ($token: expr, $pattern: pat_param) => {
        let tok = $token;

        match tok {
            Some(Token { kind: $pattern, start: _, end: _ }) => (),
            Some(token) => return Err(LangError::new_parser_unexpected_token(&token)),
            None => return Err(LangError::new_parser_end_of_file()),
        }
    };
}

#[macro_export]
macro_rules! expect_open_body {
    ($tokens: expr) => {
        expect_token!($tokens.pop(), TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Open));
    };
}

pub trait TokensExtensions {
    fn pop_err(&mut self) -> Result<Token, LangError>;
    fn peek_err(&mut self) -> Result<Token, LangError>;
}

impl TokensExtensions for Tokens {
    fn pop_err(&mut self) -> Result<Token, LangError> {
        match self.pop() {
            Some(token) => Ok(token),
            None => return Err(LangError::new_parser_end_of_file()),
        }
    }

    fn peek_err(&mut self) -> Result<Token, LangError> {
        match self.peek() {
            Some(token) => Ok(token),
            None => return Err(LangError::new_parser_end_of_file()),
        }
    }
}

impl<'a> ParserScope<'a> {
    pub fn parse_object_values(&self, tokens: &mut Tokens) -> Result<Vec<(String, ASTNode)>, LangError> {
        let mut res = Vec::new();
        let mut next_is_argument = true;
        
        loop {
            let token = match tokens.pop() {
                Some(token) => token,
                None => return Err(LangError::new_parser_end_of_file()),
            };
                
            match &token.kind {
                TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close) => break,
                TokenKind::Operator(OperatorKind::Comma) => {
                    if next_is_argument {
                        return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedParam));
                    } else {
                        next_is_argument = true;
                        continue;
                    }
                }
                token_kind => {
                    if next_is_argument {
                        next_is_argument = false;

                        // name
                        let name = match token_kind {
                            TokenKind::Symbol(name) => name,
                            _ => return Err(LangError::new_parser_unexpected_token(&token))
                        };
                        
                        // :
                        match tokens.pop() {
                            Some(Token { kind: TokenKind::Operator(OperatorKind::Colon), start: _, end: _ }) => (),
                            Some(_) => return Err(LangError::new_parser_unexpected_token(&token)),
                            None => return Err(LangError::new_parser_end_of_file()),
                        }

                        // value
                        let value = self.parse_statement(tokens)?;
                        
                        res.push((name.clone(), value));
                    } else {
                        return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedComma));
                    }
                },
            };
        }
        
        Ok(res)
    }

    /** Parses a block of tokens (something like "{ var x = 10; var y = false }").
     * It consumes only the last parenthesis and expectes the first token to be the first statement,
       in this case it will be "var"
     */ 
    pub fn parse_body(&self, tokens: &mut Tokens) -> Result<ASTBody, LangError> {
        let mut body = Vec::new();
        
        loop {
            let token = match tokens.peek() {
                Some(token) => token,
                None => break,
            };
                
            let result = match token.kind {
                TokenKind::Parenthesis(ParenthesisKind::Curly, ParenthesisState::Close) => break,
                TokenKind::NewLine => { tokens.pop(); continue },
                _ => self.parse_statement(tokens)?,
            };
            
            body.push(result);
        }
        
        // Popping the last }
        tokens.pop();
        
        Ok(body)
    }
    
    pub fn parse_vector_values(&self, tokens: &mut Tokens) -> Result<(TypeKind, ASTBody), LangError> {
        let mut body = Vec::new();
        let mut next_is_argument = true;
        let mut vector_type = TypeKind::Unknown;
        
        loop {
            let token = match tokens.peek() {
                Some(token) => token,
                None => return Err(LangError::new_parser_end_of_file()),
            };
                
            match &token.kind {
                TokenKind::Parenthesis(ParenthesisKind::Square, ParenthesisState::Close) => break,
                TokenKind::Operator(OperatorKind::Comma) => {
                    if next_is_argument {
                        return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedParam));
                    } else {
                        tokens.pop();

                        next_is_argument = true;
                        continue;
                    }
                }
                _ => {
                    if next_is_argument {
                        next_is_argument = false;
                        
                        let node = self.parse_statement(tokens)?;
                        if vector_type.is_unknown() {
                            vector_type = node.eval_type.clone();
                        } else if vector_type != node.eval_type {
                            return Err(LangError::parser(&token, ParserErrorKind::WrontType(node.eval_type.clone(), vector_type.clone())));
                        }

                        body.push(node);
                    } else {
                        return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedComma));
                    }
                },
            };
        }
        
        // Popping the last )
        tokens.pop();
        
        Ok((vector_type, body))
    }

    pub fn parse_parameter_values(&self, tokens: &mut Tokens) -> Result<ASTBody, LangError> {
        let mut body = Vec::new();
        let mut next_is_argument = true;
        
        loop {
            let token = match tokens.peek() {
                Some(token) => token,
                None => return Err(LangError::new_parser_end_of_file()),
            };
                
            match &token.kind {
                TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close) => break,
                TokenKind::Operator(OperatorKind::Comma) => {
                    if next_is_argument {
                        return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedParam));
                    } else {
                        tokens.pop();

                        next_is_argument = true;
                        continue;
                    }
                }
                _ => {
                    if next_is_argument {
                        next_is_argument = false;
                        body.push(self.parse_statement(tokens)?);
                    } else {
                        return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedComma));
                    }
                },
            };
        }
        
        // Popping the last )
        tokens.pop();
        
        Ok(body)
    }

    pub fn predict_math_result(kind: MathOperatorKind, type_a: &TypeKind, type_b: &TypeKind) -> TypeKind {
        match kind {
            MathOperatorKind::Plus => {
                match (type_a, type_b) {
                    // Int -> Int
                    (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                    
                    // Int/Float -> Float
                    (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                    (TypeKind::Float, TypeKind::Int) => TypeKind::Float,
                    
                    // Float -> Float
                    (TypeKind::Float, TypeKind::Float) => TypeKind::Float,
                    
                    // Others -> String
                    (_, _) => TypeKind::String,
                }
            },
            MathOperatorKind::Minus => {
                match (type_a, type_b) {
                    // Int -> Int
                    (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                    
                    // Int/Float -> Float
                    (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                    (TypeKind::Float, TypeKind::Int) => TypeKind::Float,
                    
                    // Float -> Float
                    (TypeKind::Float, TypeKind::Float) => TypeKind::Float,
                    
                    // Others -> String
                    (_, _) => TypeKind::Unknown,
                }
            },
            MathOperatorKind::Multiply => {
                match (type_a, type_b) {
                    // Int -> Int
                    (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                    
                    // Int/Float -> Float
                    (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                    (TypeKind::Float, TypeKind::Int) => TypeKind::Float,
                    
                    // Float -> Float
                    (TypeKind::Float, TypeKind::Float) => TypeKind::Float,
                    
                    // Others -> String
                    (_, _) => TypeKind::Unknown,
                }
            },
            MathOperatorKind::Divide => {
                match (type_a, type_b) {
                    // Int -> Int
                    (TypeKind::Int, TypeKind::Int) => TypeKind::Float,
                    
                    // Int/Float -> Float
                    (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                    (TypeKind::Float, TypeKind::Int) => TypeKind::Float,
                    
                    // Float -> Float
                    (TypeKind::Float, TypeKind::Float) => TypeKind::Float,
                    
                    // Others -> String
                    (_, _) => TypeKind::Unknown,
                }
            },
            MathOperatorKind::Modulus => {
                match (type_a, type_b) {
                    // Int -> Int
                    (TypeKind::Int, TypeKind::Int) => TypeKind::Int,
                    
                    // Int/Float -> Float
                    (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                    (TypeKind::Float, TypeKind::Int) => TypeKind::Float,
                    
                    // Float -> Float
                    (TypeKind::Float, TypeKind::Float) => TypeKind::Float,
                    
                    // Others -> String
                    (_, _) => TypeKind::Unknown,
                }
            },
            MathOperatorKind::Power => {
                match (type_a, type_b) {
                    // Int -> Int
                    (TypeKind::Int, TypeKind::Int) => TypeKind::Float,
                    
                    // Int/Float -> Float
                    (TypeKind::Int, TypeKind::Float) => TypeKind::Float,
                    (TypeKind::Float, TypeKind::Int) => TypeKind::Float,
                    
                    // Float -> Float
                    (TypeKind::Float, TypeKind::Float) => TypeKind::Float,
                    
                    // Others -> String
                    (_, _) => TypeKind::Unknown,
                }
            },
        }
    }
}

pub fn parse_type_error(tokens: &mut Tokens) -> Result<TypeKind, LangError> {
    // type
    match tokens.pop() {
        Some(Token { kind: TokenKind::Type(tk), start: _, end: _ }) => Ok(tk),
        Some(token) => Err(LangError::new_parser_unexpected_token(&token)),
        None => Err(LangError::new_parser_end_of_file()),
    }
}

pub fn parse_type_option(tokens: &mut Tokens) -> Option<TypeKind> {
    // type
    match tokens.peek() {
        Some(Token { kind: TokenKind::Type(tk), start: _, end: _ }) => {
            tokens.pop();
            Some(tk)
        },
        _ => None
    }
}

/** Parses a list of parameter names (something like "(arg0, arg1, arg2)").
 * It consumes only the last parenthesis and expectes the first token to be the first argument,
     in this case it will be "arg0"
    */ 
pub fn parse_parameter_names(tokens: &mut Tokens) -> Result<(Vec<String>, Vec<TypeKind>), LangError> {
    let mut names = Vec::new();
    let mut types = Vec::new();
    let mut next_is_argument = true;
    
    loop {
        let token = match tokens.pop() {
            Some(token) => token,
            None => return Err(LangError::new_parser_end_of_file()),
        };
        
        match &token.kind {
            TokenKind::Parenthesis(ParenthesisKind::Round, ParenthesisState::Close) => break,
            TokenKind::Symbol(name) => {
                if next_is_argument {
                    next_is_argument = false;

                    let t = parse_type_error(tokens)?;

                    names.push(name.clone());
                    types.push(t);
                } else {
                    return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedComma));
                }
            },
            TokenKind::Operator(OperatorKind::Comma) => {
                if next_is_argument {
                    return Err(LangError::parser(&token, ParserErrorKind::ParametersExpectedParam));
                } else {
                    next_is_argument = true;
                }
            },
            _ => return Err(LangError::new_parser_unexpected_token(&token)),
        };
    }

    Ok((names, types))
}