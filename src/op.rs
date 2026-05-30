#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
#[non_exhaustive]
pub enum Opcode {
	Abort,
	// 'addr'
	Load(u8),
	// Store the top value in the stack into memory @ addr
	// 'addr'
	Store(u8),
	StackDup,
	Add,
	Sub,
}
