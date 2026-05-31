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
	const FIRST: MemVal = MemVal::MAX;
	const SECOND: MemVal = 5;
	const THIRD: MemVal = FIRST.wrapping_add(SECOND);

	let mut program: Program = test_program!(
		Opcode::MemSet(0, FIRST),
		Opcode::MemSet(1, SECOND),
		Opcode::Load(2),
		Opcode::Load(3),
		Opcode::Load(0),
		Opcode::Load(1),
		Opcode::Add(Some(2)),
	);
	assert_eq!(program.run(), Status::NoFurtherInstructions);
	assert_eq!(program.step(), Status::NoFurtherInstructions);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
}

#[test]
fn sub() {
	const FIRST: MemVal = MemVal::MIN;
	const SECOND: MemVal = 5;
	const THIRD: MemVal = FIRST.wrapping_sub(SECOND);

	let mut program: Program = test_program!(
		Opcode::MemSet(0, FIRST),
		Opcode::MemSet(1, SECOND),
		Opcode::Load(2),
		Opcode::Load(3),
		Opcode::Load(0),
		Opcode::Load(1),
		Opcode::Sub(Some(2)),
	);
	assert_eq!(program.run(), Status::NoFurtherInstructions);
	assert_eq!(program.step(), Status::NoFurtherInstructions);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
}
