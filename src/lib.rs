pub mod ir;
// pub mod backends;

// pub trait Backend {
//     type Function: Function<Backend = Self>;
// }

// pub trait Function {
//     type Backend: Backend<Function = Self>;
//     type DataType;

//     fn integer(&mut self, backend: &mut Self::Backend, value: u128);

//     fn new(name: String) -> Self;
//     fn end(self, backend: &mut Self::Backend, value: Option<Self::DataType>);
// }

// pub type DataType<B> = <<B as Backend>::Function as Function>::DataType;
