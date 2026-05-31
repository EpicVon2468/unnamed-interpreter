// Group lints
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
// Specific lints
#![warn(
	clippy::as_conversions,
	clippy::min_ident_chars,
	clippy::pattern_type_mismatch,
	clippy::use_self,
	clippy::unused_trait_names,
	clippy::create_dir,
	clippy::exit,
	clippy::float_cmp,
	clippy::float_cmp_const,
	clippy::while_float,
	clippy::integer_division,
	clippy::integer_division_remainder_used,
	clippy::unreadable_literal,
	clippy::unnecessary_literal_bound,
	clippy::missing_const_for_fn,
	clippy::needless_collect,
	clippy::needless_for_each,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::infinite_loop,
	clippy::linkedlist,
	clippy::pub_use,
	clippy::wildcard_imports,
	clippy::uninlined_format_args,
	clippy::equatable_if_let,
	clippy::enum_glob_use,
	clippy::panic,
	clippy::panic_in_result_fn
)]
#![forbid(
	clippy::undocumented_unsafe_blocks,
	clippy::multiple_unsafe_ops_per_block,
	clippy::missing_safety_doc,
	unsafe_op_in_unsafe_fn,
	reason = "All unsafe code must be wrapped in one unsafe block per call, and be safety documented!"
)]
#![allow(clippy::tabs_in_doc_comments, reason = "Why???  Bad clippy!")]
#![allow(
	clippy::unnecessary_semicolon,
	reason = "Consistency & uniformity looks better!  Bad clippy!"
)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	reason = "I'll get to writing doc comments when I get to them."
)]
#![allow(
	clippy::default_trait_access,
	clippy::upper_case_acronyms,
	reason = "Shush"
)]
#![allow(clippy::borrowed_box)]
#![feature(const_trait_impl, const_default, derive_const, const_cmp)]
#![doc = include_str!("../README.md")]
pub mod err;
pub mod op;
#[cfg(test)]
pub mod tests;

use std::collections::VecDeque;
use std::hint::{cold_path, unreachable_unchecked};
use std::io::stdin;
use std::str::FromStr as _;

use crate::err::Status;
use crate::op::Opcode;

pub fn main() {
	let mut program: Program = const { Program::default() };
	let _ = program.run();
	dbg!(program.get_char(Some(0)));
	dbg!(program.get_int(Some(1)));
	dbg!(program.mem);
}

pub type MemAddr = u8;
pub type MemVal = u32;
const _: () = assert!(
	size_of::<MemAddr>() <= size_of::<usize>(),
	"MemAddr is used to index an array, and therefore cannot be larger than `usize`!",
);
#[allow(
	clippy::absurd_extreme_comparisons,
	clippy::cast_sign_loss,
	clippy::as_conversions,
	clippy::cast_possible_truncation,
	unused_comparisons
)]
const _: () = assert!(
	(f32::NEG_INFINITY as MemAddr) >= 0,
	"MemAddr must be unsigned!",
);

#[derive(Debug)]
pub struct Program {
	// MemAddr : MemVal
	pub(crate) mem: [MemVal; 32],
	// stack : MemVal
	pub(crate) stack: VecDeque<MemVal>,
	pub(crate) instructions: VecDeque<Opcode>,
}

#[rustfmt::skip]
#[allow(
	clippy::derivable_impls,
	reason = "VecDeque::new() is const, but VecDeque::default() is not"
)]
impl const Default for Program {
	fn default() -> Self {
		Self {
			mem: [0; _],
			stack: VecDeque::new(),
			instructions: VecDeque::new(),
		}
	}
}

impl Program {
	fn stack_push(&mut self, value: MemVal) {
		assert!(self.stack.len() < 16, "Tried to grow stack beyond maximum!");
		self.stack.push_back(value);
	}

	#[must_use]
	pub fn collect_parameters(&mut self, n: usize) -> Vec<MemVal> {
		self.stack.drain((self.stack.len() - n)..).collect()
	}
}

impl Program {
	#[must_use]
	pub fn run(&mut self) -> Status {
		loop {
			propagate!(self.step());
		}
	}

	pub fn step(&mut self) -> Status {
		if self.instructions.is_empty() {
			return Status::NoFurtherInstructions;
		};
		let opcode: Opcode = self.instructions.pop_front().unwrap_or_else(|| {
			cold_path();
			// SANITY: [`self.instructions`] is not empty at this point, therefore this is unreachable.
			// SAFETY:
			// Problem(s):
			// - `unreachable_unchecked()` is unsafe, and it is Undefined Behaviour for it to be reached.
			// Excuse(s):
			// - This statement cannot be reached.
			unsafe {
				unreachable_unchecked();
			};
		});
		self.execute(&opcode)
	}
}

impl Program {
	pub fn execute(&mut self, opcode: &Opcode) -> Status {
		match *opcode {
			Opcode::Abort => return Status::ProgramAbort,
			Opcode::Load(addr) => self.load(addr),
			Opcode::Store(addr) => self.store(addr),
			Opcode::StackDup => self.stack_dup(),
			Opcode::StackPop => {
				let _ = self.stack_pop();
			},
			Opcode::MemSet(addr, value) => self.mem_set(addr, value),
			Opcode::MemClear => self.mem_clear(),
			Opcode::GetChar(dest) => propagate!(self.get_char(dest)),
			Opcode::GetInt(dest) => propagate!(self.get_int(dest)),
			Opcode::Add(dest) => self.add(dest),
			Opcode::Sub(dest) => self.sub(dest),
			Opcode::Mul(dest) => self.mul(dest),
			Opcode::Div(dest) => self.div(dest),
		};
		Status::OK
	}

	pub fn load(&mut self, addr: MemAddr) {
		self.stack_push(self.mem[usize::from(addr)]);
	}

	pub fn store(&mut self, addr: MemAddr) {
		// can't inline this variable due to borrowing rules
		let value: MemVal = self.stack_pop();
		self.mem_set(addr, value);
	}

	// TODO: should this push_back `0` if no value already exists?
	pub fn stack_dup(&mut self) {
		let Some(value): Option<&MemVal> = self.stack.back() else {
			// This isn't technically unlikely to occur, but sane programs should only be using stackdup when a value actually exists.
			cold_path();
			return;
		};
		self.stack.push_back(*value);
	}

	#[must_use]
	fn stack_pop(&mut self) -> MemVal {
		self.stack.pop_back().expect("Stack was empty!")
	}

	pub fn mem_set(&mut self, addr: MemAddr, value: MemVal) {
		self.mem[usize::from(addr)] = value;
	}

	pub const fn mem_clear(&mut self) {
		self.mem = [0; _];
	}

	pub fn get_char(&mut self, dest: Option<MemAddr>) -> Status {
		let mut buf: String = String::new();
		// SANITY: Only empty on `EOF`, otherwise it would be `\n` or `\r\n`.
		if stdin().read_line(&mut buf).is_err() || buf.is_empty() {
			return Status::InvalidInput;
		};
		// SAFETY:
		// Problem(s):
		// - Dereferencing pointers is unsafe.
		// - If `buf` is empty, a segmentation fault occurs.
		// Excuse(s):
		// - `std` ensures the pointer being dereferenced is properly aligned, is not null, and is not invalid.
		// - The enclosing function contains checks which perform an early return if `buf` is empty.
		let value: u8 = unsafe {
			*buf.trim().as_ptr()
		};
		dbg!(value);
		self.op_result_store(dest, MemVal::from(value));
		Status::OK
	}

	pub fn get_int(&mut self, dest: Option<MemAddr>) -> Status {
		let mut buf: String = String::new();
		if stdin().read_line(&mut buf).is_err() || buf.is_empty() {
			return Status::InvalidInput;
		};
		let Ok(value): Result<MemVal, _> = MemVal::from_str(buf.trim()) else {
			return Status::InvalidInput;
		};
		self.op_result_store(dest, value);
		Status::OK
	}

	pub fn add(&mut self, dest: Option<MemAddr>) {
		let (lhs, rhs): (MemVal, MemVal) = self.collect_int_params();
		self.op_result_store(dest, lhs.wrapping_add(rhs));
	}

	pub fn sub(&mut self, dest: Option<MemAddr>) {
		let (lhs, rhs): (MemVal, MemVal) = self.collect_int_params();
		self.op_result_store(dest, lhs.wrapping_sub(rhs));
	}

	pub fn mul(&mut self, dest: Option<MemAddr>) {
		let (lhs, rhs): (MemVal, MemVal) = self.collect_int_params();
		self.op_result_store(dest, lhs.wrapping_mul(rhs));
	}

	pub fn div(&mut self, dest: Option<MemAddr>) {
		let (lhs, rhs): (MemVal, MemVal) = self.collect_int_params();
		self.op_result_store(dest, lhs.wrapping_div(rhs));
	}

	fn op_result_store(&mut self, dest: Option<MemAddr>, value: MemVal) {
		if let Some(dest) = dest {
			self.mem_set(dest, value);
		} else {
			self.stack_push(value);
		};
	}

	#[must_use]
	fn collect_int_params(&mut self) -> (MemVal, MemVal) {
		let [lhs, rhs]: [MemVal] = *self.collect_parameters(2) else {
			cold_path();
			// SANITY: [`Self::collect_parameters()`] would've panicked if it couldn't collect exactly two parameters, therefore this is unreachable.
			// SAFETY:
			// Problem(s):
			// - `unreachable_unchecked()` is unsafe, and it is Undefined Behaviour for it to be reached.
			// Excuse(s):
			// - This statement cannot be reached.
			unsafe {
				unreachable_unchecked();
			};
		};
		(lhs, rhs)
	}
}
