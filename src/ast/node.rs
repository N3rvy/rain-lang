use crate::common::lang_value::LangValue;


pub type ASTBody = Vec<ASTNode>;
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
    },
    Literal {
        value: LangValue,
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
    
    pub fn new_function_invok(variable: ASTChild) -> ASTChild {
        Box::new(ASTNode::FunctionInvok { variable })
    }
    
    pub fn new_literal(value: LangValue) -> ASTChild {
        Box::new(ASTNode::Literal { value })
    }
}