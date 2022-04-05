use common::ast::types::{MathOperatorKind, BoolOperatorKind};
use wasm_encoder::{ValType, Instruction};
use crate::build_code::FunctionBuilder;

impl<'a, 'b> FunctionBuilder<'a, 'b> {
    pub fn build_math_op(&mut self, op: &MathOperatorKind, left: ValType, right: ValType) {
        self.check_type_convert(left, right);

        let inst = match op {
            MathOperatorKind::Plus => Self::build_add_op(left),
            MathOperatorKind::Minus => Self::build_sub_op(left),
            MathOperatorKind::Multiply => Self::build_mul_op(left),
            MathOperatorKind::Divide => Self::build_div_op(left),
            MathOperatorKind::Modulus => todo!(),
            MathOperatorKind::Power => todo!(),
        };

        self.instructions.push(inst);
    }

    pub fn build_bool_op(&mut self, op: &BoolOperatorKind, left: ValType, right: ValType) {
        self.check_type_convert(left, right);

        let inst = match op {
            BoolOperatorKind::Equal => Self::build_eq_op(left),
            BoolOperatorKind::Different => Self::build_ne_op(left),
            BoolOperatorKind::Bigger => Self::build_gt_op(left),
            BoolOperatorKind::Smaller => Self::build_lt_op(left),
            BoolOperatorKind::BiggerEq => Self::build_ge_op(left),
            BoolOperatorKind::SmallerEq => Self::build_le_op(left),
        };

        self.instructions.push(inst);
    }

    fn build_le_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32LeS,
            ValType::I64 => Instruction::I64LeS,
            ValType::F32 => Instruction::F32Le,
            ValType::F64 => Instruction::F64Le,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    fn build_ge_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32GeS,
            ValType::I64 => Instruction::I64GeS,
            ValType::F32 => Instruction::F32Ge,
            ValType::F64 => Instruction::F64Ge,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    fn build_lt_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32LtS,
            ValType::I64 => Instruction::I64LtS,
            ValType::F32 => Instruction::F32Lt,
            ValType::F64 => Instruction::F64Lt,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    fn build_gt_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32GtS,
            ValType::I64 => Instruction::I64GtS,
            ValType::F32 => Instruction::F32Gt,
            ValType::F64 => Instruction::F64Gt,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    fn build_ne_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Ne,
            ValType::I64 => Instruction::I64Ne,
            ValType::F32 => Instruction::F32Ne,
            ValType::F64 => Instruction::F64Ne,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    fn build_eq_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Eq,
            ValType::I64 => Instruction::I64Eq,
            ValType::F32 => Instruction::F32Eq,
            ValType::F64 => Instruction::F64Eq,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    fn check_type_convert(&mut self, left: ValType, right: ValType) {
        if left != right {
            match Self::convert_op(left, right) {
                Some(op) => self.instructions.push(op),
                None => (),
            }
        }
    }

    fn convert_op(left: ValType, right: ValType) -> Option<Instruction<'static>> {
        match left {
            ValType::I32 => {
                match right {
                    ValType::I32 => None,
                    ValType::I64 => Some(Instruction::I32WrapI64),
                    ValType::F32 => Some(Instruction::I32TruncF32S),
                    ValType::F64 => Some(Instruction::I32TruncF64S),
                    ValType::V128 => todo!(),
                    ValType::FuncRef => todo!(),
                    ValType::ExternRef => todo!(),
                }
            },
            ValType::I64 => {
                match right {
                    ValType::I32 => Some(Instruction::I64ExtendI32S),
                    ValType::I64 => None,
                    ValType::F32 => Some(Instruction::I64TruncF32S),
                    ValType::F64 => Some(Instruction::I64TruncF64S),
                    ValType::V128 => todo!(),
                    ValType::FuncRef => todo!(),
                    ValType::ExternRef => todo!(),
                }
            },
            ValType::F32 => {
                match right {
                    ValType::I32 => Some(Instruction::F32ConvertI32S),
                    ValType::I64 => Some(Instruction::F32ConvertI64S),
                    ValType::F32 => None,
                    ValType::F64 => Some(Instruction::F32DemoteF64),
                    ValType::V128 => todo!(),
                    ValType::FuncRef => todo!(),
                    ValType::ExternRef => todo!(),
                }
            },
            ValType::F64 => {
                match right {
                    ValType::I32 => Some(Instruction::F64ConvertI32S),
                    ValType::I64 => Some(Instruction::F64ConvertI64S),
                    ValType::F32 => Some(Instruction::F64PromoteF32),
                    ValType::F64 => None,
                    ValType::V128 => todo!(),
                    ValType::FuncRef => todo!(),
                    ValType::ExternRef => todo!(),
                }
            },
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    pub fn build_sub_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Sub,
            ValType::I64 => Instruction::I64Sub,
            ValType::F32 => Instruction::F32Sub,
            ValType::F64 => Instruction::F64Sub,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    pub fn build_mul_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Mul,
            ValType::I64 => Instruction::I64Mul,
            ValType::F32 => Instruction::F32Mul,
            ValType::F64 => Instruction::F64Mul,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    pub fn build_div_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32DivS,
            ValType::I64 => Instruction::I64DivS,
            ValType::F32 => Instruction::F32Div,
            ValType::F64 => Instruction::F64Div,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }

    pub fn build_add_op(type_: ValType) -> Instruction<'static> {
        match type_ {
            ValType::I32 => Instruction::I32Add,
            ValType::I64 => Instruction::I64Add,
            ValType::F32 => Instruction::F32Add,
            ValType::F64 => Instruction::F64Add,
            ValType::V128 => todo!(),
            ValType::FuncRef => todo!(),
            ValType::ExternRef => todo!(),
        }
    }
}