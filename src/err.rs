#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
#[must_use]
#[non_exhaustive]
pub enum Status {
	OK,
	ProgramAbort,
	ProgramComplete,
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
