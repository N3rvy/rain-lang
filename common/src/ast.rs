use crate::{lang_value::LangValue, types::{MathOperatorKind, BoolOperatorKind, ReturnKind}};

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
    VariableAsgn {
        name: String,
        value: ASTChild,
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
        value: Option<ASTChild>,
        kind: ReturnKind,
    },
    IfStatement {
        condition: ASTChild,
        body: ASTBody,
    },
    ForStatement {
        left: ASTChild,
        right: ASTChild,
        body: ASTBody,
        iter_name: String,
    },
    WhileStatement {
        condition: ASTChild,
        body: ASTBody,
    },
    FieldAccess {
        variable: ASTChild,
        field_name: String,
    },
    VectorLiteral {
        values: Vec<ASTChild>
    },
    ValueFieldAccess {
        variable: ASTChild,
        value: ASTChild,
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
    
    pub fn new_variable_asgn(name: String, value: ASTChild) -> ASTChild {
        Box::new(ASTNode::VariableAsgn { name, value })
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
    
    pub fn new_return_statement(value: Option<ASTChild>, kind: ReturnKind) -> ASTChild {
        Box::new(ASTNode::ReturnStatement { value, kind })
    }
    
    pub fn new_if_statement(condition: ASTChild, body: ASTBody) -> ASTChild {
        Box::new(ASTNode::IfStatement { condition, body })
    }
    
    pub fn new_for_statement(left: ASTChild, right: ASTChild, body: ASTBody, iter_name: String) -> ASTChild {
        Box::new(ASTNode::ForStatement { left, right, body, iter_name })
    }
    
    pub fn new_while_statement(condition: ASTChild, body: ASTBody) -> ASTChild {
        Box::new(ASTNode::WhileStatement { condition, body })
    }

    pub fn new_field_access(variable: ASTChild, field_name: String) -> ASTChild {
        Box::new(ASTNode::FieldAccess { variable, field_name })
    }

    pub fn new_vector_literal(values: Vec<ASTChild>) -> ASTChild {
        Box::new(ASTNode::VectorLiteral { values })
    }
 
    pub fn new_value_field_access(variable: ASTChild, value: ASTChild) -> ASTChild {
        Box::new(ASTNode::ValueFieldAccess { variable, value })
    }
}