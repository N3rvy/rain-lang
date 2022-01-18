use crate::{lang_value::LangValue, types::{MathOperatorKind, BoolOperatorKind, ReturnKind}};


pub type ASTBody = Vec<ASTNode>;

pub struct ASTNode {
    pub kind: Box<NodeKind>,
    pub eval_type: TypeKind,
}

impl ASTNode {
    pub fn new(kind: NodeKind, eval_type: TypeKind) -> Self {
        Self {
            kind: Box::new(kind),
            eval_type,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    Unknown,
    Int,
    Float,
    String,
    Nothing,
}

pub enum NodeKind {
    Root {
        body: ASTBody,
    },
    VariableDecl {
        name: String,
        value: ASTNode,
    },
    VaraibleRef {
        name: String,
    },
    VariableAsgn {
        name: String,
        value: ASTNode,
    },
    FunctionInvok {
        variable: ASTNode,
        parameters: ASTBody,
    },
    MethodInvok {
        object: ASTNode,
        name: String,
        parameters: ASTBody,
    },
    Literal {
        value: LangValue,
    },
    MathOperation {
        operation: MathOperatorKind,
        left: ASTNode,
        right: ASTNode,
    },
    BoolOperation {
        operation: BoolOperatorKind,
        left: ASTNode,
        right: ASTNode,
    },
    ReturnStatement {
        value: Option<ASTNode>,
        kind: ReturnKind,
    },
    IfStatement {
        condition: ASTNode,
        body: ASTBody,
    },
    ForStatement {
        left: ASTNode,
        right: ASTNode,
        body: ASTBody,
        iter_name: String,
    },
    WhileStatement {
        condition: ASTNode,
        body: ASTBody,
    },
    FieldAccess {
        variable: ASTNode,
        field_name: String,
    },
    VectorLiteral {
        values: Vec<ASTNode>
    },
    ObjectLiteral {
        values: Vec<(String, ASTNode)>,
    },
    ValueFieldAccess {
        variable: ASTNode,
        value: ASTNode,
    },
    Import {
        identifier: String,
    }
}

impl NodeKind {
    pub fn new_root(body: ASTBody) -> NodeKind {
        NodeKind::Root { body }
    }
    
    pub fn new_variable_decl(name: String, value: ASTNode) -> NodeKind {
        NodeKind::VariableDecl { name, value }
    }
    
    pub fn new_variable_ref(name: String) -> NodeKind {
        NodeKind::VaraibleRef { name }
    }
    
    pub fn new_variable_asgn(name: String, value: ASTNode) -> NodeKind {
        NodeKind::VariableAsgn { name, value }
    }
    
    pub fn new_function_invok(variable: ASTNode, parameters: ASTBody) -> NodeKind {
        NodeKind::FunctionInvok { variable, parameters }
    }
    
    pub fn new_method_invok(object: ASTNode, name: String, parameters: ASTBody) -> NodeKind {
        NodeKind::MethodInvok { object, name, parameters }
    }
    
    pub fn new_literal(value: LangValue) -> NodeKind {
        NodeKind::Literal { value }
    }
    
    pub fn new_math_operation(operation: MathOperatorKind, left: ASTNode, right: ASTNode) -> NodeKind {
        NodeKind::MathOperation { operation, left, right }
    }

    pub fn new_bool_operation(operation: BoolOperatorKind, left: ASTNode, right: ASTNode) -> NodeKind {
        NodeKind::BoolOperation { operation, left, right }
    }
    
    pub fn new_return_statement(value: Option<ASTNode>, kind: ReturnKind) -> NodeKind {
        NodeKind::ReturnStatement { value, kind }
    }
    
    pub fn new_if_statement(condition: ASTNode, body: ASTBody) -> NodeKind {
        NodeKind::IfStatement { condition, body }
    }
    
    pub fn new_for_statement(left: ASTNode, right: ASTNode, body: ASTBody, iter_name: String) -> NodeKind {
        NodeKind::ForStatement { left, right, body, iter_name }
    }
    
    pub fn new_while_statement(condition: ASTNode, body: ASTBody) -> NodeKind {
        NodeKind::WhileStatement { condition, body }
    }

    pub fn new_field_access(variable: ASTNode, field_name: String) -> NodeKind {
        NodeKind::FieldAccess { variable, field_name }
    }

    pub fn new_vector_literal(values: Vec<ASTNode>) -> NodeKind {
        NodeKind::VectorLiteral { values }
    }
    
    pub fn new_object_literal(values: Vec<(String, ASTNode)>) -> NodeKind {
        NodeKind::ObjectLiteral { values }
    }
 
    pub fn new_value_field_access(variable: ASTNode, value: ASTNode) -> NodeKind {
        NodeKind::ValueFieldAccess { variable, value }
    }

    pub fn new_import(identifier: String) -> NodeKind {
        NodeKind::Import { identifier }
    }
}