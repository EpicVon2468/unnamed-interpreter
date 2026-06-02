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
fn mem_set() {
	const FIRST: MemVal = 1;
	const SECOND: MemVal = MemVal::MAX;
	const THIRD: MemVal = MemVal::MIN;

	let mut program: Program = test_program!(
		Opcode::MemSet(0, FIRST),
		Opcode::MemSet(1, SECOND),
		Opcode::MemSet(2, THIRD),
	);
	assert_eq!(program.mem, [0; _]);
	assert_eq!(program.run(), Status::ProgramComplete);
	assert_eq!(program.step(), Status::ProgramComplete);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
	assert_eq!(program.mem[3], 0);
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
		Opcode::Add(Some(3)),
	);
	assert_eq!(program.run(), Status::ProgramComplete);
	assert_eq!(program.step(), Status::ProgramComplete);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
	assert_eq!(program.mem[3], 0);
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
		Opcode::Sub(Some(3)),
	);
	assert_eq!(program.run(), Status::ProgramComplete);
	assert_eq!(program.step(), Status::ProgramComplete);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
	assert_eq!(program.mem[3], 0);
}

#[test]
fn mul() {
	const FIRST: MemVal = MemVal::MAX;
	const SECOND: MemVal = 2;
	const THIRD: MemVal = FIRST.wrapping_mul(SECOND);

	let mut program: Program = test_program!(
		Opcode::MemSet(0, FIRST),
		Opcode::MemSet(1, SECOND),
		Opcode::Load(2),
		Opcode::Load(3),
		Opcode::Load(0),
		Opcode::Load(1),
		Opcode::Mul(Some(2)),
		Opcode::Mul(Some(3)),
	);
	assert_eq!(program.run(), Status::ProgramComplete);
	assert_eq!(program.step(), Status::ProgramComplete);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
	assert_eq!(program.mem[3], 0);
}

#[test]
fn div() {
	const FIRST: MemVal = MemVal::MAX;
	const SECOND: MemVal = 2;
	const THIRD: MemVal = FIRST.wrapping_div(SECOND);

	let mut program: Program = test_program!(
		Opcode::MemSet(0, FIRST),
		Opcode::MemSet(1, SECOND),
		Opcode::MemSet(3, 1),
		Opcode::Load(2),
		Opcode::Load(3),
		Opcode::Load(0),
		Opcode::Load(1),
		Opcode::Div(Some(2)),
		Opcode::Div(Some(3)),
	);
	assert_eq!(program.run(), Status::ProgramComplete);
	assert_eq!(program.step(), Status::ProgramComplete);
	assert_eq!(program.mem[0], FIRST);
	assert_eq!(program.mem[1], SECOND);
	assert_eq!(program.mem[2], THIRD);
	assert_eq!(program.mem[3], 0);
}
