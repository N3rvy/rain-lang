use std::collections::HashMap;
use std::sync::Arc;
use common::ast::parsing_types::{ParsableFunctionType, ParsableType};
use common::ast::types::{ClassType, FunctionType, TypeKind};
use common::errors::{LangError, ParserErrorKind};
use common::module::ModuleUID;
use common::tokens::{Token, TokenKind};
use crate::modules::module_loader::GlobalDeclarationKind;
use crate::parser_scope::ParserScope;

pub enum ScopeGetResult {
    Class(ModuleUID, Arc<ClassType>),
    Ref(ModuleUID, TypeKind),
    None,
}

enum GlobalKind {
    Var(ModuleUID, TypeKind),
    Func(ModuleUID, FunctionType),
    Class(ModuleUID, Arc<ClassType>),
}

pub struct ParserModuleScope {
    pub uid: ModuleUID,
    globals: HashMap<String, GlobalKind>,
}

impl ParserModuleScope {
    pub fn new(module_uid: ModuleUID) -> Self {
        Self {
            uid: module_uid,
            globals: HashMap::new(),
        }
    }

    pub fn new_child(&self) -> ParserScope {
        ParserScope::new_module_child(self)
    }

    pub fn get(&self, name: &String) -> ScopeGetResult {
        match self.globals.get(name) {
            Some(GlobalKind::Var(uid, type_)) => ScopeGetResult::Ref(*uid, type_.clone()),
            Some(GlobalKind::Func(uid, type_))
                => ScopeGetResult::Ref(*uid, TypeKind::Function(type_.clone())),
            Some(GlobalKind::Class(uid, type_)) => ScopeGetResult::Class(*uid, type_.clone()),
            None => ScopeGetResult::None,
        }
    }

    pub fn convert_parsable_func_type(&self, func_type: &ParsableFunctionType) -> Result<FunctionType, LangError> {
        let mut params = Vec::new();

        for param in &func_type.0 {
            params.push(self.convert_parsable_type(param)?);
        }

        Ok(FunctionType(params, Box::new(self.convert_parsable_type(&func_type.1)?)))
    }

    pub fn convert_parsable_type(&self, type_: &ParsableType) -> Result<TypeKind, LangError> {
        Ok(match type_ {
            ParsableType::Unknown => TypeKind::Unknown,
            ParsableType::Nothing => TypeKind::Nothing,
            ParsableType::Int => TypeKind::Int,
            ParsableType::Float => TypeKind::Float,
            ParsableType::Bool => TypeKind::Bool,
            ParsableType::String => TypeKind::String,
            ParsableType::Vector(type_) => TypeKind::Vector(Box::new(self.convert_parsable_type(type_.as_ref())?)),
            ParsableType::Function(ParsableFunctionType(params, return_type)) => {
                let mut params_types = Vec::new();

                for param in params {
                    params_types.push(self.convert_parsable_type(param)?);
                }

                TypeKind::Function(FunctionType(
                    params_types,
                    Box::new(self.convert_parsable_type(return_type)?)))
            },
            ParsableType::Custom(name) => {
                // TODO: This need a token position in case of error

                match self.globals.get(name) {
                    Some(GlobalKind::Var(_, type_)) => type_.clone(),
                    _ => return Err(LangError::parser(
                        &Token::new(TokenKind::Symbol(name.clone()), 0, 0),
                        ParserErrorKind::UnexpectedError(
                            "convert_parsable_type: custom type not found".to_string()))),
                }
            },
        })
    }

    pub fn declare_var(&mut self, name: String, type_kind: TypeKind) {
        self.globals
            .insert(name, GlobalKind::Var(self.uid, type_kind));
    }

    pub fn declare_func(&mut self, name: String, func_type: FunctionType) {
        self.globals
            .insert(name, GlobalKind::Func(self.uid, func_type));
    }

    pub fn declare_class(&mut self, name: String, class_type: Arc<ClassType>) {
        self.globals
            .insert(name, GlobalKind::Class(self.uid, class_type));
    }

    pub fn declare_external_func(&mut self, name: String, module: ModuleUID, func_type: FunctionType) {
        self.globals
            .insert(name, GlobalKind::Func(module, func_type));
    }

    pub fn declare_external_var(&mut self, name: String, module: ModuleUID, type_: TypeKind) {
        self.globals
            .insert(name, GlobalKind::Var(module, type_));
    }

    pub fn declare_external_class(&mut self, name: String, module: ModuleUID, class_type: Arc<ClassType>) {
        self.globals
            .insert(name, GlobalKind::Class(module, class_type));
    }
}