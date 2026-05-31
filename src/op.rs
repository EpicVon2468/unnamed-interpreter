use crate::{MemAddr, MemVal};

#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
#[non_exhaustive]
pub enum Opcode {
	Abort,
	/// Load the [`literal value`][`MemVal`] onto the stack.
	Load(MemAddr),
	/// Pop the top value in the stack, storing it into memory at `addr`.
	Store(MemAddr),
	/// Duplicate the head of the stack.
	StackDup,
	StackPop,
	/// Store the [`literal value`][`MemValue`] into memory at `addr`.
	MemSet(MemAddr, MemVal),
	/// Clears all memory.  This is more efficient than manually using [`Self::MemSet`] to clear every value.
	MemClear,
	/// Read 1 `char` from the standard input.
	GetChar(Option<MemAddr>),
	GetInt(Option<MemAddr>),
	Add(Option<MemAddr>),
	Sub(Option<MemAddr>),
	Mul(Option<MemAddr>),
	Div(Option<MemAddr>),
}
