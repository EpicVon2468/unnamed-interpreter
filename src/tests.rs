#![cfg(test)]
use crate::err::Status;
use crate::op::Opcode;
use crate::{MemVal, Program};

macro_rules! test_program {
	($($opcode:expr),* $(,)?) => {
		Program {
			instructions: [
				$($opcode,)*
			].into(),
			..const { Default::default() }
		}
	};
}

#[test]
fn add() {
	let mut program: Program = test_program!(
		Opcode::Load(0),
		Opcode::Load(1),
		Opcode::Add(None),
		Opcode::Store(2),
	);
	program.mem[0] = MemVal::MAX;
	program.mem[1] = 5;
	assert_eq!(program.run(), Status::NoFurtherInstructions);
	assert_eq!(program.step(), Status::NoFurtherInstructions,);
	assert_eq!(program.mem[0], MemVal::MAX);
	assert_eq!(program.mem[1], 5);
	assert_eq!(program.mem[2], 4);
}
