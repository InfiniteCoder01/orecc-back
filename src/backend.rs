#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Integer {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
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

pub trait Backend {
    type DataType;

    // * Data types
    fn integer(&mut self, value: Integer) -> Self::DataType;

    // * Operations
    fn binary_operation(
        &mut self,
        operation: BinaryOperation,
        lhs: Self::DataType,
        rhs: Self::DataType,
    ) -> Self::DataType;

    // * Automatics
    fn unsigned_autosize(&mut self, value: u128) -> Self::DataType {
        match u128::BITS - value.leading_zeros() {
            0..=8 => self.integer(Integer::U8(value as _)),
            9..=16 => self.integer(Integer::U16(value as _)),
            17..=32 => self.integer(Integer::U32(value as _)),
            33..=64 => self.integer(Integer::U64(value as _)),
            _ => self.integer(Integer::U128(value)),
        }
    }
}
