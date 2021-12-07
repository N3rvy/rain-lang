use crate::ast::node::ASTNode;

pub enum LangValue {
    String(String),
    Int(i32),
    Float(f32),
    Number(f32),
    Bool(bool),
    Function(Box<ASTNode>),
}