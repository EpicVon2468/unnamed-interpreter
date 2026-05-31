#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
pub enum Status {
	OK,
	ProgramAbort,
	NoFurtherInstructions,
	InvalidInput,
}

#[macro_export]
macro_rules! propagate {
	($function_call:expr $(,)?) => {{
		let status: Status = $function_call;
		if status != Status::OK {
			return status;
		};
	}};
}
