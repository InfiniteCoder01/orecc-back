#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Nop,
}

/// Typecast rules:
/// Number to number - just cast
/// Same size - transmutate
/// Different size - panic if implicit, crop or pad with 0's if explicit
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Void,
    Unsigned(u16),
    Integer(u16),
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    DivisionReminder,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalAnd,
    LogicalOr,
    ShiftLeft,
    ShiftRight,
}

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
}

impl Function {
    pub fn new(name: String, return_type: Type) -> Self {
        Self {
            name,
            return_type,
            assembly: Vec::new(),
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
        write!(f, "function {}() -> {}:", self.name, self.return_type)?;
        Ok(())
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nop => write!(f, "nop"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Unsigned(size) => write!(f, "u{}", size * 8),
            Type::Integer(size) => write!(f, "i{}", size * 8),
        }
    }
}
