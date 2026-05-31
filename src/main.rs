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
use std::ops::ControlFlow;

use crate::err::Status;
use crate::op::Opcode;

pub fn main() {
	let mut program: Program = const { Program::default() };
	program.run();
	dbg!(program);
}

pub type MemAddr = u8;
pub type MemVal = u64;
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
			mem: [0; 32],
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

	pub fn collect_parameters(&mut self, n: usize) -> Vec<MemVal> {
		let mut params: Vec<MemVal> = Vec::with_capacity(n);
		{
			let mut index: usize = 0;
			while index < n {
				params.push(self.stack_pop());
				index += 1;
			}
		};
		params
	}
}

impl Program {
	pub fn run(&mut self) {
		loop {
			let ControlFlow::Break(status): ControlFlow<Status> = self.step() else {
				continue;
			};
			if status == Status::NoFurtherInstructions {
				break;
			};
		}
	}

	pub fn step(&mut self) -> ControlFlow<Status> {
		if self.instructions.is_empty() {
			brk!(NoFurtherInstructions);
		};
		let opcode: Opcode = self
			.instructions
			.pop_front()
			// SAFETY: We've already determined that self.instructions is not empty.
			.unwrap_or_else(|| unreachable!());
		self.execute(&opcode)
	}
}

impl Program {
	pub fn execute(&mut self, opcode: &Opcode) -> ControlFlow<Status> {
		match *opcode {
			Opcode::Abort => brk!(ProgramAbort),
			Opcode::Load(addr) => self.load(addr),
			Opcode::Store(addr) => self.store(addr),
			Opcode::StackDup => self.stack_dup(),
			Opcode::StackPop => {
				self.stack_pop();
			},
			Opcode::MemSet(addr, value) => self.mem_set(addr, value),
			Opcode::GetChar => self.get_char(),
			Opcode::Add => self.add(),
			Opcode::Sub => self.sub(),
		};
		ControlFlow::Continue(())
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

	fn stack_pop(&mut self) -> MemVal {
		self.stack.pop_back().expect("Stack was empty!")
	}

	pub fn mem_set(&mut self, addr: MemAddr, value: MemVal) {
		self.mem[usize::from(addr)] = value;
	}

	pub fn get_char(&mut self) {
		#[allow(unreachable_code, clippy::diverging_sub_expression)]
		self.stack_push(todo!("`fgetc(stdin)`"));
	}

	pub fn add(&mut self) {
		let (lhs, rhs): (MemVal, MemVal) = self.collect_int_params();
		self.stack_push(lhs.wrapping_add(rhs));
	}

	pub fn sub(&mut self) {
		let (lhs, rhs): (MemVal, MemVal) = self.collect_int_params();
		self.stack_push(lhs.wrapping_sub(rhs));
	}

	fn collect_int_params(&mut self) -> (MemVal, MemVal) {
		let [lhs, rhs]: [MemVal] = *self.collect_parameters(2) else {
			cold_path();
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
