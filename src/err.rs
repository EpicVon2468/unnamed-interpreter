#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
pub enum Status {
	ProgramAbort,
	NoFurtherInstructions,
}

// can't name it 'break' :(
#[macro_export]
macro_rules! brk {
	($err:ident $(,)?) => {
		return std::ops::ControlFlow::Break($crate::err::Status::$err)
	};
}
