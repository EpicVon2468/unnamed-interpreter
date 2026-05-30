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

use std::collections::VecDeque;
use std::hint::{cold_path, unreachable_unchecked};
use std::ops::ControlFlow;

use crate::err::Status;
use crate::op::Opcode;

pub fn main() {
	let mut program: Program = Program {
		instructions: [
			Opcode::Load(0),
			Opcode::Load(1),
			Opcode::Add,
			Opcode::Store(2),
			Opcode::Abort,
		]
		.into(),
		..Default::default()
	};
	program.mem[0] = MemVal::MAX;
	program.mem[1] = 5;
	program.run();
	dbg!(program);
}

type MemVal = u64;

#[derive(Debug)]
pub struct Program {
	// mem_addr : value
	pub(crate) mem: [MemVal; 32],
	// stack : value
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
		self.stack.push_back(value);
	}

	fn stack_pop(&mut self) -> MemVal {
		self.stack.pop_back().expect("Stack was empty!")
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
			let ControlFlow::Break(status) = dbg!(self.step()) else {
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
			Opcode::StackDup => self.stackdup(),
			Opcode::Add => self.add(),
			Opcode::Sub => self.sub(),
		};
		ControlFlow::Continue(())
	}

	pub fn load(&mut self, addr: u8) {
		self.stack_push(self.mem[usize::from(addr)]);
	}

	pub fn store(&mut self, addr: u8) {
		self.mem[usize::from(addr)] = self.stack_pop();
	}

	pub fn stackdup(&mut self) {
		let Some(value): Option<&MemVal> = self.stack.back() else {
			return;
		};
		self.stack.push_back(*value);
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
