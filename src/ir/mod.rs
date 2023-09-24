use bitvec::vec::BitVec;
mod instruction;
pub use instruction::*;

// * ---------------------------------- Structures ---------------------------------- * //
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
    name: String,
    return_type: Type,
    pub assembly: Vec<Instruction>,
    allocated: BitVec,
}

impl Function {
    pub fn new(name: String, return_type: Type) -> Self {
        Self {
            name,
            return_type,
            assembly: Vec::new(),
            allocated: BitVec::new(),
        }
    }

}

// * ------------------------------------ Display ----------------------------------- * //
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in &self.functions {
            write!(f, "{function}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "function {}() -> {}:", self.name, self.return_type)?;
        for instruction in &self.assembly {
            writeln!(f, "    {instruction}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BinaryOp(target, op, lhs, rhs) => write!(f, "%{target} = {op} {lhs}, {rhs}"),
            Self::Return(value) => write!(f, "return {value}"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Unsigned(size) => write!(f, "u{}", size * 8),
            Self::Integer(size) => write!(f, "i{}", size * 8),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allocated(index) => write!(f, "%{index}"),
            Self::Unsigned(value) => write!(f, "{value}"),
            Self::Integer(value) => write!(f, "{value}"),
        }
    }
}

impl std::fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Subtract => write!(f, "sub"),
            Self::Multiply => write!(f, "mul"),
            Self::Divide => write!(f, "div"),
            Self::DivisionReminder => write!(f, "rem"),
            Self::BitwiseAnd => write!(f, "and"),
            Self::BitwiseOr => write!(f, "or"),
            Self::BitwiseXor => write!(f, "xor"),
            Self::LogicalAnd => write!(f, "land"),
            Self::LogicalOr => write!(f, "lor"),
            Self::ShiftLeft => write!(f, "shl"),
            Self::ShiftRight => write!(f, "shr"),
        }
    }
}
