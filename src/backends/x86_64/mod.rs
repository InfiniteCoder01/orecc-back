use crate::*;

#[derive(Debug, Default)]
pub struct X86_64 {
    functions: Vec<Function>,
}

impl Backend for X86_64 {
    type Function = Function;
}

// * ----------------------------------- Functions ---------------------------------- * //
#[derive(Debug)]
pub struct Function {
    name: String,
    assembly: Vec<Instruction>,
}

impl crate::Function for Function {
    type Backend = X86_64;
    type DataType = Data;

    fn integer(&mut self, backend: &mut Self::Backend, value: u128) {
        todo!()
    }

    fn new(name: String) -> Self {
        Self {
            name,
            assembly: Vec::new(),
        }
    }

    fn end(self, backend: &mut X86_64, tail_return: Option<Data>) {
        backend.functions.push(self);
    }
}

#[derive(Debug)]
pub enum Data {
    // Value(Integer),
    Allocated(usize),
}

#[derive(Debug)]
pub enum Instruction {
    Label(String),
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

impl Function {
    // fn allocate(&mut self) -> usize {
    //     self.allocated += 1;
    //     self.allocated - 1
    // }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Data::Value(value) => match value {
            //     Integer::U8(value) => write!(f, "{value} as u8"),
            //     Integer::U16(value) => write!(f, "{value} as u16"),
            //     Integer::U32(value) => write!(f, "{value} as u32"),
            //     Integer::U64(value) => write!(f, "{value} as u64"),
            //     Integer::U128(value) => write!(f, "{value} as u128"),
            //     Integer::I8(value) => write!(f, "{value} as i8"),
            //     Integer::I16(value) => write!(f, "{value} as i16"),
            //     Integer::I32(value) => write!(f, "{value} as i32"),
            //     Integer::I64(value) => write!(f, "{value} as i64"),
            //     Integer::I128(value) => write!(f, "{value} as i128"),
            // },
            Data::Allocated(cell) => write!(f, "#{cell}"),
        }
    }
}

impl std::fmt::Display for X86_64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in &self.functions {
            writeln!(f, "{}:", function.name)?;
            for instruction in &function.assembly {
                match instruction {
                    Instruction::Label(name) => writeln!(f, "{name}:")?,
                    Instruction::Mov(dst, val) => writeln!(f, "    mov #{dst}, {val}")?,

                    Instruction::LogicalAnd(dst, val) => writeln!(f, "    and #{dst}, {val}")?,
                    Instruction::LogicalOr(dst, val) => writeln!(f, "    or #{dst}, {val}")?,
                    Instruction::Add(dst, val) => writeln!(f, "    add #{dst}, {val}")?,
                    Instruction::Subtract(dst, val) => writeln!(f, "    sub #{dst}, {val}")?,
                    Instruction::Multiply(dst, val) => writeln!(f, "    mul #{dst}, {val}")?,
                    Instruction::Divide(dst, val) => writeln!(f, "    div #{dst}, {val}")?,
                    Instruction::DivisionReminder(dst, val) => {
                        writeln!(f, "    rem #{dst}, {val}")?
                    }
                    Instruction::BitwiseAnd(dst, val) => writeln!(f, "    and #{dst}, {val}")?,
                    Instruction::BitwiseOr(dst, val) => writeln!(f, "    or #{dst}, {val}")?,
                    Instruction::BitwiseXor(dst, val) => writeln!(f, "    xor #{dst}, {val}")?,
                    Instruction::ShiftLeft(dst, val) => writeln!(f, "    shl #{dst}, {val}")?,
                    Instruction::ShiftRight(dst, val) => writeln!(f, "    shr #{dst}, {val}")?,
                }
            }
        }
        Ok(())
    }
}
