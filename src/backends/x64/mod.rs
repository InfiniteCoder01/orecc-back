use crate::*;

#[derive(Debug, Default)]
pub struct X64Backend {
    allocated: usize,
    assembly: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Data {
    Value(Integer),
    Allocated(usize),
}

#[derive(Debug)]
pub enum Instruction {
    Mov(usize, Data),

    Add(usize, Data),
    Subtract(usize, Data),
    Multiply(usize, Data),
    Divide(usize, Data),
    DivisionReminder(usize, Data),
    BitwiseAnd(usize, Data),
    BitwiseOr(usize, Data),
    BitwiseXor(usize, Data),
    LogicalAnd(usize, Data),
    LogicalOr(usize, Data),
    ShiftLeft(usize, Data),
    ShiftRight(usize, Data),
}

impl Backend for X64Backend {
    type DataType = Data;

    fn integer(&mut self, value: Integer) -> Self::DataType {
        Data::Value(value)
    }

    fn binary_operation(
        &mut self,
        operation: BinaryOperation,
        lhs: Self::DataType,
        rhs: Self::DataType,
    ) -> Self::DataType {
        let lhs = match lhs {
            Data::Allocated(lhs) => lhs,
            lhs => {
                let allocated_lhs = self.allocate();
                self.assembly.push(Instruction::Mov(allocated_lhs, lhs));
                allocated_lhs
            }
        };

        self.assembly.push(match operation {
            BinaryOperation::Add => Instruction::Add(lhs, rhs),
            BinaryOperation::Subtract => Instruction::Subtract(lhs, rhs),
            BinaryOperation::Multiply => Instruction::Multiply(lhs, rhs),
            BinaryOperation::Divide => Instruction::Divide(lhs, rhs),
            BinaryOperation::DivisionReminder => Instruction::DivisionReminder(lhs, rhs),
            BinaryOperation::BitwiseAnd => Instruction::BitwiseAnd(lhs, rhs),
            BinaryOperation::BitwiseOr => Instruction::BitwiseOr(lhs, rhs),
            BinaryOperation::BitwiseXor => Instruction::BitwiseXor(lhs, rhs),
            BinaryOperation::LogicalAnd => Instruction::LogicalAnd(lhs, rhs),
            BinaryOperation::LogicalOr => Instruction::LogicalOr(lhs, rhs),
            BinaryOperation::ShiftLeft => Instruction::ShiftLeft(lhs, rhs),
            BinaryOperation::ShiftRight => Instruction::ShiftRight(lhs, rhs),
        });

        Data::Allocated(lhs)
    }
}

impl X64Backend {
    fn allocate(&mut self) -> usize {
        self.allocated += 1;
        self.allocated - 1
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Value(value) => match value {
                Integer::U8(value) => write!(f, "{value} as u8"),
                Integer::U16(value) => write!(f, "{value} as u16"),
                Integer::U32(value) => write!(f, "{value} as u32"),
                Integer::U64(value) => write!(f, "{value} as u64"),
                Integer::U128(value) => write!(f, "{value} as u128"),
                Integer::I8(value) => write!(f, "{value} as i8"),
                Integer::I16(value) => write!(f, "{value} as i16"),
                Integer::I32(value) => write!(f, "{value} as i32"),
                Integer::I64(value) => write!(f, "{value} as i64"),
                Integer::I128(value) => write!(f, "{value} as i128"),
            },
            Data::Allocated(cell) => write!(f, "#{cell}"),
        }
    }
}

impl std::fmt::Display for X64Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in &self.assembly {
            match instruction {
                Instruction::Mov(dst, val) => writeln!(f, "mov #{dst}, {val}")?,

                Instruction::LogicalAnd(dst, val) => writeln!(f, "and #{dst}, {val}")?,
                Instruction::LogicalOr(dst, val) => writeln!(f, "or #{dst}, {val}")?,
                Instruction::Add(dst, val) => writeln!(f, "add #{dst}, {val}")?,
                Instruction::Subtract(dst, val) => writeln!(f, "sub #{dst}, {val}")?,
                Instruction::Multiply(dst, val) => writeln!(f, "mul #{dst}, {val}")?,
                Instruction::Divide(dst, val) => writeln!(f, "div #{dst}, {val}")?,
                Instruction::DivisionReminder(dst, val) => writeln!(f, "rem #{dst}, {val}")?,
                Instruction::BitwiseAnd(dst, val) => writeln!(f, "and #{dst}, {val}")?,
                Instruction::BitwiseOr(dst, val) => writeln!(f, "or #{dst}, {val}")?,
                Instruction::BitwiseXor(dst, val) => writeln!(f, "xor #{dst}, {val}")?,
                Instruction::ShiftLeft(dst, val) => writeln!(f, "shl #{dst}, {val}")?,
                Instruction::ShiftRight(dst, val) => writeln!(f, "shr #{dst}, {val}")?,
            }
        }
        Ok(())
    }
}
