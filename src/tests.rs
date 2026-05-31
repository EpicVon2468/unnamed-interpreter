#![cfg(test)]
use std::ops::ControlFlow;

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
		Opcode::Add,
		Opcode::Store(2),
		Opcode::Abort,
	);
	program.mem[0] = MemVal::MAX;
	program.mem[1] = 5;
	program.run();
	assert_eq!(
		program.step(),
		ControlFlow::Break(Status::NoFurtherInstructions)
	);
	assert_eq!(program.mem[0], MemVal::MAX);
	assert_eq!(program.mem[1], 5);
	assert_eq!(program.mem[2], 4);
}
