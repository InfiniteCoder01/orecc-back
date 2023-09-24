use super::Function;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    BinaryOp(usize, BinaryOperation, Value, Value),
    Return(Value),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Allocated(usize),
    Unsigned(u128),
    Integer(i128),
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

impl Function {
    pub fn binary_op(&mut self, op: BinaryOperation, lhs: Value, rhs: Value) -> Value {
        if let Value::Allocated(allocated) = lhs {
            self.drop(allocated);
        }
        if let Value::Allocated(allocated) = rhs {
            self.drop(allocated);
        }
        let output = self.allocate();
        self.assembly
            .push(Instruction::BinaryOp(output, op, lhs, rhs));
        Value::Allocated(output)
    }

    pub fn allocate(&mut self) -> usize {
        self.allocated.first_zero().unwrap_or_else(|| {
            self.allocated.push(true);
            self.allocated.len() - 1
        })
    }

    pub fn drop(&mut self, allocated: usize) {
        self.allocated.set(allocated, false);
    }
}
