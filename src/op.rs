use crate::{MemAddr, MemVal};

#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
#[non_exhaustive]
pub enum Opcode {
	Abort,
	/// Load the [`literal value`][`MemVal`] onto the stack.
	Load(MemAddr),
	/// Store the top value in the stack into memory at `addr`.
	Store(MemAddr),
	/// Duplicate the head of the stack.
	StackDup,
	StackPop,
	/// Store the [`literal value`][`MemValue`] into memory at `addr`.
	MemSet(MemAddr, MemVal),
	GetChar,
	Add,
	Sub,
	Mul,
	Div,
}
