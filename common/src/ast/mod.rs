use std::sync::Arc;
use crate::ast::types::ClassType;
use crate::module::ModuleUID;

use self::types::{TypeKind, LiteralKind, MathOperatorKind, BoolOperatorKind, ReturnKind, Function};

pub mod types;


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

    pub fn new_empty() -> Self {
        Self {
            kind: Box::new(NodeKind::Literal {
                value: LiteralKind::Nothing,
            }),
            eval_type: TypeKind::Nothing,
        }
    }
}

pub enum NodeKind {
    VariableDecl {
        name: String,
        value: ASTNode,
    },
    VariableRef {
        module: ModuleUID,
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
    Literal {
        value: LiteralKind,
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
    FunctionLiteral {
        value: Arc<Function>,
    },
    ValueFieldAccess {
        variable: ASTNode,
        value: ASTNode,
    },
    ConstructClass {
        type_: Arc<ClassType>,
    }
}

impl NodeKind {
    pub fn new_variable_decl(name: String, value: ASTNode) -> NodeKind {
        NodeKind::VariableDecl { name, value }
    }
    
    pub fn new_variable_ref(module: ModuleUID, name: String) -> NodeKind {
        NodeKind::VariableRef { module, name }
    }
    
    pub fn new_variable_asgn(name: String, value: ASTNode) -> NodeKind {
        NodeKind::VariableAsgn { name, value }
    }
    
    pub fn new_function_invok(variable: ASTNode, parameters: ASTBody) -> NodeKind {
        NodeKind::FunctionInvok { variable, parameters }
    }
    
    pub fn new_literal(value: LiteralKind) -> NodeKind {
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
    
    pub fn new_function_literal(value: Arc<Function>) -> NodeKind {
        NodeKind::FunctionLiteral { value }
    }
 
    pub fn new_value_field_access(variable: ASTNode, value: ASTNode) -> NodeKind {
        NodeKind::ValueFieldAccess { variable, value }
    }

    pub fn new_construct_class(type_: Arc<ClassType>) -> NodeKind {
        NodeKind::ConstructClass { type_ }
    }
}
