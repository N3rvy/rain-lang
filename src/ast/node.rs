use crate::{common::lang_value::LangValue, tokenizer::tokens::{MathOperatorKind, BoolOperatorKind}};


pub type ASTBody = Vec<ASTChild>;
pub type ASTChild = Box<ASTNode>;

pub enum ASTNode {
    Root {
        body: ASTBody,
    },
    VariableDecl {
        name: String,
        value: ASTChild,
    },
    VaraibleRef {
        name: String,
    },
    FunctionInvok {
        variable: ASTChild,
        parameters: ASTBody,
    },
    Literal {
        value: LangValue,
    },
    MathOperation {
        operation: MathOperatorKind,
        left: ASTChild,
        right: ASTChild,
    },
    BoolOperation {
        operation: BoolOperatorKind,
        left: ASTChild,
        right: ASTChild,
    },
    ReturnStatement {
        value: ASTChild,
    },
    IfStatement {
        condition: ASTChild,
        body: ASTBody,
    },
}

impl ASTNode {
    pub fn new_root(body: ASTBody) -> ASTChild {
        Box::new(ASTNode::Root { body })
    }
    
    pub fn new_variable_decl(name: String, value: ASTChild) -> ASTChild {
        Box::new(ASTNode::VariableDecl { name, value })
    }
    
    pub fn new_variable_ref(name: String) -> ASTChild {
        Box::new(ASTNode::VaraibleRef { name })
    }
    
    pub fn new_function_invok(variable: ASTChild, parameters: ASTBody) -> ASTChild {
        Box::new(ASTNode::FunctionInvok { variable, parameters })
    }
    
    pub fn new_literal(value: LangValue) -> ASTChild {
        Box::new(ASTNode::Literal { value })
    }
    
    pub fn new_math_operation(operation: MathOperatorKind, left: ASTChild, right: ASTChild) -> ASTChild {
        Box::new(ASTNode::MathOperation { operation, left, right })
    }

    pub fn new_bool_operation(operation: BoolOperatorKind, left: ASTChild, right: ASTChild) -> ASTChild {
        Box::new(ASTNode::BoolOperation { operation, left, right })
    }
    
    pub fn new_return_statement(value: ASTChild) -> ASTChild {
        Box::new(ASTNode::ReturnStatement { value })
    }
    
    pub fn new_if_statement(condition: ASTChild, body: ASTBody) -> ASTChild {
        Box::new(ASTNode::IfStatement { condition, body })
    }
}