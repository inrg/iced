/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

#![allow(dead_code)] //TODO: REMOVE

use super::icedconstants::IcedConstants;
use super::{instructionmemorysizes, instructionopcounts, Code, CodeSize, MemorySize, Mnemonic, OpKind, Register, RoundingControl};
use std::fmt;
use std::mem;

struct MemoryFlags;
impl MemoryFlags {
	pub(crate) const SCALE_MASK: u32 = 3;
	pub(crate) const DISPL_SIZE_SHIFT: u32 = 2;
	pub(crate) const DISPL_SIZE_MASK: u32 = 7;
	pub(crate) const SEGMENT_PREFIX_SHIFT: u32 = 5;
	pub(crate) const SEGMENT_PREFIX_MASK: u32 = 7;
	// Unused bits here
	pub(crate) const BROADCAST: u32 = 0x8000;
}

struct OpKindFlags;
impl OpKindFlags {
	pub(crate) const OP_KIND_BITS: u32 = 5;
	pub(crate) const OP_KIND_MASK: u32 = (1 << Self::OP_KIND_BITS) - 1;
	pub(crate) const OP1_KIND_SHIFT: u32 = 5;
	pub(crate) const OP2_KIND_SHIFT: u32 = 10;
	pub(crate) const OP3_KIND_SHIFT: u32 = 15;
	pub(crate) const DATA_LENGTH_MASK: u32 = 0xF;
	pub(crate) const DATA_LENGTH_SHIFT: u32 = 20;
	// Unused bits here
	pub(crate) const CODE_SIZE_MASK: u32 = 3;
	pub(crate) const CODE_SIZE_SHIFT: u32 = 30;

	// Bits ignored by ==/!=
	pub(crate) const EQUALS_IGNORE_MASK: u32 = Self::CODE_SIZE_MASK << Self::CODE_SIZE_SHIFT;
}

struct CodeFlags;
impl CodeFlags {
	pub(crate) const CODE_BITS: u32 = 13;
	pub(crate) const CODE_MASK: u32 = (1 << Self::CODE_BITS) - 1;
	pub(crate) const ROUNDING_CONTROL_MASK: u32 = 7;
	pub(crate) const ROUNDING_CONTROL_SHIFT: u32 = 13;
	pub(crate) const OPMASK_MASK: u32 = 7;
	pub(crate) const OPMASK_SHIFT: u32 = 16;
	pub(crate) const INSTR_LENGTH_MASK: u32 = 0xF;
	pub(crate) const INSTR_LENGTH_SHIFT: u32 = 19;
	// Unused bits here
	pub(crate) const SUPPRESS_ALL_EXCEPTIONS: u32 = 0x0200_0000;
	pub(crate) const ZEROING_MASKING: u32 = 0x0400_0000;
	pub(crate) const XACQUIRE_PREFIX: u32 = 0x0800_0000;
	pub(crate) const XRELEASE_PREFIX: u32 = 0x1000_0000;
	pub(crate) const REPE_PREFIX: u32 = 0x2000_0000;
	pub(crate) const REPNE_PREFIX: u32 = 0x4000_0000;
	pub(crate) const LOCK_PREFIX: u32 = 0x8000_0000;

	// Bits ignored by ==/!=
	pub(crate) const EQUALS_IGNORE_MASK: u32 = Self::INSTR_LENGTH_MASK << Self::INSTR_LENGTH_SHIFT;
}

/// A 16/32/64-bit x86 instruction
#[derive(Copy, Clone, Default)]
pub struct Instruction {
	next_rip: u64,
	code_flags: u32,    // CodeFlags
	op_kind_flags: u32, // OpKindFlags
	// If it's a 64-bit immediate/offset/target, the high 32 bits is in mem_displ
	immediate: u32,
	// This is the high 32 bits if it's a 64-bit immediate/offset/target
	mem_displ: u32,
	memory_flags: u16, // MemoryFlags
	mem_base_reg: u8,  // Register
	mem_index_reg: u8, // Register
	reg0: u8,          // Register
	reg1: u8,          // Register
	reg2: u8,          // Register
	reg3: u8,          // Register
}

#[allow(clippy::len_without_is_empty)]
impl Instruction {
	/// Creates an empty `Instruction` (all fields are cleared)
	#[inline]
	pub fn new() -> Self {
		Default::default()
	}

	/// Checks if two instructions are equal, comparing all bits, not ignoring anything. `==` ignores some fields.
	pub fn eq_all_bits(&self, other: Self) -> bool {
		self.next_rip == other.next_rip
			&& self.code_flags == other.code_flags
			&& self.op_kind_flags == other.op_kind_flags
			&& self.immediate == other.immediate
			&& self.mem_displ == other.mem_displ
			&& self.memory_flags == other.memory_flags
			&& self.mem_base_reg == other.mem_base_reg
			&& self.mem_index_reg == other.mem_index_reg
			&& self.reg0 == other.reg0
			&& self.reg1 == other.reg1
			&& self.reg2 == other.reg2
			&& self.reg3 == other.reg3
	}

	/// Gets the 16-bit IP of the instruction
	#[inline]
	pub fn ip16(&self) -> u16 {
		self.next_rip as u16 - self.len() as u16
	}

	/// Sets the 16-bit IP of the instruction
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_ip16(&mut self, new_value: u16) {
		self.next_rip = new_value as u64 + self.len() as u64;
	}

	/// Gets the 32-bit IP of the instruction
	#[inline]
	pub fn ip32(&self) -> u32 {
		self.next_rip as u32 - self.len() as u32
	}

	/// Sets the 32-bit IP of the instruction
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_ip32(&mut self, new_value: u32) {
		self.next_rip = new_value as u64 + self.len() as u64;
	}

	/// Gets the 64-bit IP of the instruction
	#[inline]
	pub fn ip(&self) -> u64 {
		self.next_rip - self.len() as u64
	}

	/// Sets the 64-bit IP of the instruction
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_ip(&mut self, new_value: u64) {
		self.next_rip = new_value + self.len() as u64;
	}

	/// Gets the 16-bit IP of the next instruction
	#[inline]
	pub fn next_ip16(&self) -> u16 {
		self.next_rip as u16
	}

	/// Sets the 16-bit IP of the next instruction
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_next_ip16(&mut self, new_value: u16) {
		self.next_rip = new_value as u64;
	}

	/// Gets the 32-bit IP of the next instruction
	#[inline]
	pub fn next_ip32(&self) -> u32 {
		self.next_rip as u32
	}

	/// Sets the 32-bit IP of the next instruction
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_next_ip32(&mut self, new_value: u32) {
		self.next_rip = new_value as u64;
	}

	/// Gets the 64-bit IP of the next instruction
	#[inline]
	pub fn next_ip(&self) -> u64 {
		self.next_rip
	}

	/// Sets the 64-bit IP of the next instruction
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_next_ip(&mut self, new_value: u64) {
		self.next_rip = new_value;
	}

	/// Gets the code size when the instruction was decoded. This value is informational and can
	/// be used by a formatter.
	#[inline]
	pub fn code_size(&self) -> CodeSize {
		// safe: the bits are only initialized to valid values by iced and
		// we store 2 bits in op_kind_flags, which is exactly the size of CodeSize
		unsafe { mem::transmute(((self.op_kind_flags >> OpKindFlags::CODE_SIZE_SHIFT) & OpKindFlags::CODE_SIZE_MASK) as u8) }
	}

	/// Sets the code size when the instruction was decoded. This value is informational and can
	/// be used by a formatter.
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_code_size(&mut self, new_value: CodeSize) {
		self.op_kind_flags = (self.op_kind_flags & !(OpKindFlags::CODE_SIZE_MASK << OpKindFlags::CODE_SIZE_SHIFT))
			| (((new_value as u32) & OpKindFlags::CODE_SIZE_MASK) << OpKindFlags::CODE_SIZE_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_code_size(&mut self, new_value: CodeSize) {
		self.op_kind_flags |= (new_value as u32) << OpKindFlags::CODE_SIZE_SHIFT;
	}

	/// Gets the instruction code, see also `mnemonic()`
	#[inline]
	pub fn code(&self) -> Code {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.code_flags & CodeFlags::CODE_MASK) }
	}

	/// Sets the instruction code
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_code(&mut self, new_value: Code) {
		self.code_flags = (self.code_flags & !CodeFlags::CODE_MASK) | (new_value as u32);
	}

	#[inline]
	pub(crate) fn internal_set_code(&mut self, new_value: Code) {
		self.code_flags |= new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_code_no_check(&mut self, new_value: Code) {
		self.code_flags = (self.code_flags & !CodeFlags::CODE_MASK) | new_value as u32;
	}

	/// Gets the mnemonic, see also `code()`
	#[inline]
	pub fn mnemonic(&self) -> Mnemonic {
		self.code().to_mnemonic()
	}

	/// Gets the operand count. An instruction can have 0-5 operands.
	#[inline]
	pub fn op_count(&self) -> i32 {
		instructionopcounts::OP_COUNT[(self.code_flags & CodeFlags::CODE_MASK) as usize] as i32
	}

	/// Gets the length of the instruction, 0-15 bytes. This is just informational. If you modify the instruction
	/// or create a new one, this property could return the wrong value.
	#[inline]
	pub fn len(&self) -> i32 {
		((self.code_flags >> CodeFlags::INSTR_LENGTH_SHIFT) & CodeFlags::INSTR_LENGTH_MASK) as i32
	}

	/// Sets the length of the instruction, 0-15 bytes. This is just informational. If you modify the instruction
	/// or create a new one, this property could return the wrong value.
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	pub fn set_len(&mut self, new_value: i32) {
		self.code_flags = (self.code_flags & !(CodeFlags::INSTR_LENGTH_MASK << CodeFlags::INSTR_LENGTH_SHIFT))
			| (((new_value as u32) & CodeFlags::INSTR_LENGTH_MASK) << CodeFlags::INSTR_LENGTH_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_len(&mut self, new_value: u32) {
		self.code_flags |= new_value << CodeFlags::INSTR_LENGTH_SHIFT;
	}

	#[inline]
	pub(crate) fn internal_has_repe_prefix_has_xrelease_prefix(&self) -> bool {
		(self.code_flags & (CodeFlags::REPE_PREFIX | CodeFlags::XRELEASE_PREFIX)) != 0
	}

	#[inline]
	pub(crate) fn internal_has_repne_prefix_has_xacquire_prefix(&self) -> bool {
		(self.code_flags & (CodeFlags::REPNE_PREFIX | CodeFlags::XACQUIRE_PREFIX)) != 0
	}

	#[inline]
	pub(crate) fn internal_has_repe_or_repne_prefix(&self) -> bool {
		(self.code_flags & (CodeFlags::REPE_PREFIX | CodeFlags::REPNE_PREFIX)) != 0
	}

	/// Checks if the instruction has the XACQUIRE prefix (F2)
	#[inline]
	pub fn has_xacquire_prefix(&self) -> bool {
		(self.code_flags & CodeFlags::XACQUIRE_PREFIX) != 0
	}

	/// Sets the has XACQUIRE prefix (F2) flag
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_has_xacquire_prefix(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::XACQUIRE_PREFIX;
		} else {
			self.code_flags &= !CodeFlags::XACQUIRE_PREFIX;
		}
	}

	#[inline]
	pub(crate) fn internal_set_has_xacquire_prefix(&mut self) {
		self.code_flags |= CodeFlags::XACQUIRE_PREFIX
	}

	/// Checks if the instruction has the XRELEASE prefix (F3)
	#[inline]
	pub fn has_xrelease_prefix(&self) -> bool {
		(self.code_flags & CodeFlags::XRELEASE_PREFIX) != 0
	}

	/// Sets the has XRELEASE prefix (F3) flag
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_has_xrelease_prefix(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::XRELEASE_PREFIX;
		} else {
			self.code_flags &= !CodeFlags::XRELEASE_PREFIX;
		}
	}

	#[inline]
	pub(crate) fn internal_set_has_xrelease_prefix(&mut self) {
		self.code_flags |= CodeFlags::XRELEASE_PREFIX
	}

	/// Checks if the instruction has the REPE or REP prefix (F3)
	#[inline]
	pub fn has_rep_prefix(&self) -> bool {
		(self.code_flags & CodeFlags::REPE_PREFIX) != 0
	}

	/// Sets the has REPE or REP prefix (F3) flag
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_has_rep_prefix(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::REPE_PREFIX;
		} else {
			self.code_flags &= !CodeFlags::REPE_PREFIX;
		}
	}

	/// Checks if the instruction has the REPE or REP prefix (F3)
	#[inline]
	pub fn has_repe_prefix(&self) -> bool {
		(self.code_flags & CodeFlags::REPE_PREFIX) != 0
	}

	/// Sets the has REPE or REP prefix (F3) flag
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_has_repe_prefix(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::REPE_PREFIX;
		} else {
			self.code_flags &= !CodeFlags::REPE_PREFIX;
		}
	}

	#[inline]
	pub(crate) fn internal_set_has_repe_prefix(&mut self) {
		self.code_flags |= CodeFlags::REPE_PREFIX
	}

	#[inline]
	pub(crate) fn internal_clear_has_repe_prefix(&mut self) {
		self.code_flags &= !CodeFlags::REPE_PREFIX
	}

	/// Checks if the instruction has the REPNE prefix (F2)
	#[inline]
	pub fn has_repne_prefix(&self) -> bool {
		(self.code_flags & CodeFlags::REPNE_PREFIX) != 0
	}

	/// Sets the has REPNE prefix (F2) flag
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_has_repne_prefix(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::REPNE_PREFIX;
		} else {
			self.code_flags &= !CodeFlags::REPNE_PREFIX;
		}
	}

	#[inline]
	pub(crate) fn internal_set_has_repne_prefix(&mut self) {
		self.code_flags |= CodeFlags::REPNE_PREFIX
	}

	#[inline]
	pub(crate) fn internal_clear_has_repne_prefix(&mut self) {
		self.code_flags &= !CodeFlags::REPNE_PREFIX
	}

	/// Checks if the instruction has the LOCK prefix (F0)
	#[inline]
	pub fn has_lock_prefix(&self) -> bool {
		(self.code_flags & CodeFlags::LOCK_PREFIX) != 0
	}

	/// Sets the has LOCK prefix (F0) flag
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_has_lock_prefix(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::LOCK_PREFIX;
		} else {
			self.code_flags &= !CodeFlags::LOCK_PREFIX;
		}
	}

	#[inline]
	pub(crate) fn internal_set_has_lock_prefix(&mut self) {
		self.code_flags |= CodeFlags::LOCK_PREFIX
	}

	#[inline]
	pub(crate) fn internal_clear_has_lock_prefix(&mut self) {
		self.code_flags &= !CodeFlags::LOCK_PREFIX
	}

	/// Gets operand #0's kind if the operand exists (see `op_count()`)
	#[inline]
	pub fn op0_kind(&self) -> OpKind {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute((self.op_kind_flags & OpKindFlags::OP_KIND_MASK) as u8) }
	}

	/// Sets operand #0's kind if the operand exists (see `op_count()`)
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_op0_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags = (self.op_kind_flags & !OpKindFlags::OP_KIND_MASK) | ((new_value as u32) & OpKindFlags::OP_KIND_MASK);
	}

	#[inline]
	pub(crate) fn internal_set_op0_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags |= new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_op0_is_not_reg_or_op0_is_not_reg(&self) -> bool {
		(self.op_kind_flags & (OpKindFlags::OP_KIND_MASK | (OpKindFlags::OP_KIND_MASK << OpKindFlags::OP1_KIND_SHIFT))) != 0
	}

	/// Gets operand #1's kind if the operand exists (see `op_count()`)
	#[inline]
	pub fn op1_kind(&self) -> OpKind {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(((self.op_kind_flags >> OpKindFlags::OP1_KIND_SHIFT) & OpKindFlags::OP_KIND_MASK) as u8) }
	}

	/// Sets operand #1's kind if the operand exists (see `op_count()`)
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_op1_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags = (self.op_kind_flags & !(OpKindFlags::OP_KIND_MASK << OpKindFlags::OP1_KIND_SHIFT))
			| (((new_value as u32) & OpKindFlags::OP_KIND_MASK) << OpKindFlags::OP1_KIND_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_op1_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags |= (new_value as u32) << OpKindFlags::OP1_KIND_SHIFT;
	}

	/// Gets operand #2's kind if the operand exists (see `op_count()`)
	#[inline]
	pub fn op2_kind(&self) -> OpKind {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(((self.op_kind_flags >> OpKindFlags::OP2_KIND_SHIFT) & OpKindFlags::OP_KIND_MASK) as u8) }
	}

	/// Sets operand #2's kind if the operand exists (see `op_count()`)
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_op2_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags = (self.op_kind_flags & !(OpKindFlags::OP_KIND_MASK << OpKindFlags::OP2_KIND_SHIFT))
			| (((new_value as u32) & OpKindFlags::OP_KIND_MASK) << OpKindFlags::OP2_KIND_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_op2_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags |= (new_value as u32) << OpKindFlags::OP2_KIND_SHIFT;
	}

	/// Gets operand #3's kind if the operand exists (see `op_count()`)
	#[inline]
	pub fn op3_kind(&self) -> OpKind {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(((self.op_kind_flags >> OpKindFlags::OP3_KIND_SHIFT) & OpKindFlags::OP_KIND_MASK) as u8) }
	}

	/// Sets operand #3's kind if the operand exists (see `op_count()`)
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_op3_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags = (self.op_kind_flags & !(OpKindFlags::OP_KIND_MASK << OpKindFlags::OP3_KIND_SHIFT))
			| (((new_value as u32) & OpKindFlags::OP_KIND_MASK) << OpKindFlags::OP3_KIND_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_op3_kind(&mut self, new_value: OpKind) {
		self.op_kind_flags |= (new_value as u32) << OpKindFlags::OP3_KIND_SHIFT;
	}

	/// Gets operand #4's kind if the operand exists (see `op_count()`)
	#[inline]
	pub fn op4_kind(&self) -> OpKind {
		OpKind::Immediate8
	}

	/// Sets operand #4's kind if the operand exists (see `op_count()`)
	///
	/// # Arguments
	///
	/// * `new_value`: new value
	#[inline]
	pub fn set_op4_kind(&mut self, new_value: OpKind) {
		if new_value != OpKind::Immediate8 {
			panic!("NYI"); //TODO:
		}
	}

	#[inline]
	pub(crate) fn internal_set_op4_kind(&mut self, new_value: OpKind) {
		if new_value != OpKind::Immediate8 {
			panic!("NYI"); //TODO:
		}
	}

	/// Gets an operand's kind if it exists (see `op_count()`)
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	pub fn op_kind(&self, operand: i32) -> OpKind {
		match operand {
			0 => self.op0_kind(),
			1 => self.op1_kind(),
			2 => self.op2_kind(),
			3 => self.op3_kind(),
			4 => self.op4_kind(),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Sets an operand's kind
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	/// * `op_kind`: Operand kind
	pub fn set_op_kind(&mut self, operand: i32, op_kind: OpKind) {
		match operand {
			0 => self.set_op0_kind(op_kind),
			1 => self.set_op1_kind(op_kind),
			2 => self.set_op2_kind(op_kind),
			3 => self.set_op3_kind(op_kind),
			4 => self.set_op4_kind(op_kind),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets the segment override prefix or `Register::None` if none. See also `memory_segment()`.
	/// Use this property if the operand has kind `OpKind::Memory`, `OpKind::Memory64`,
	/// `OpKind::MemorySegSI`, `OpKind::MemorySegESI`, `OpKind::MemorySegRSI`
	pub fn segment_prefix(&self) -> Register {
		let index = (((self.memory_flags as u32) >> MemoryFlags::SEGMENT_PREFIX_SHIFT) & MemoryFlags::SEGMENT_PREFIX_MASK) - 1;
		if index < 6 {
			// safe: it's guaranteed to only return one of ES,CS,SS,DS,FS,GS
			unsafe { mem::transmute((Register::ES as u32 + index) as u8) }
		} else {
			Register::None
		}
	}

	/// Sets the segment override prefix or `Register::None` if none. See also `memory_segment()`.
	/// Use this property if the operand has kind `OpKind::Memory`, `OpKind::Memory64`,
	/// `OpKind::MemorySegSI`, `OpKind::MemorySegESI`, `OpKind::MemorySegRSI`
	///
	/// # Arguments
	///
	/// * `new_value`: Segment register prefix
	pub fn set_segment_prefix(&mut self, new_value: Register) {
		let enc_value = if new_value == Register::None {
			0
		} else {
			(((new_value as u32) - (Register::ES as u32)) + 1) & MemoryFlags::SEGMENT_PREFIX_MASK
		};
		self.memory_flags = (((self.memory_flags as u32) & !(MemoryFlags::SEGMENT_PREFIX_MASK << MemoryFlags::SEGMENT_PREFIX_SHIFT))
			| (enc_value << MemoryFlags::SEGMENT_PREFIX_SHIFT)) as u16;
	}

	/// Gets the effective segment register used to reference the memory location.
	/// Use this property if the operand has kind `OpKind::Memory`, `OpKind::Memory64`,
	/// `OpKind::MemorySegSI`, `OpKind::MemorySegESI`, `OpKind::MemorySegRSI`
	pub fn memory_segment(&self) -> Register {
		let seg_reg = self.segment_prefix();
		if seg_reg != Register::None {
			return seg_reg;
		}
		match self.memory_base() {
			Register::BP | Register::EBP | Register::ESP | Register::RBP | Register::RSP => Register::SS,
			_ => Register::DS,
		}
	}

	/// Gets the size of the memory displacement in bytes. Valid values are 0, 1 (16/32/64-bit), 2 (16-bit), 4 (32-bit), 8 (64-bit).
	/// Note that the return value can be 1 and `memory_displacement()` may still not fit in
	/// a signed byte if it's an EVEX encoded instruction.
	/// Use this property if the operand has kind `OpKind::Memory`
	pub fn memory_displ_size(&self) -> i32 {
		let size: u32 = ((self.memory_flags as u32) >> MemoryFlags::DISPL_SIZE_SHIFT) & MemoryFlags::DISPL_SIZE_MASK;
		if size <= 2 {
			size as i32
		} else if size == 3 {
			4
		} else {
			8
		}
	}

	/// Sets the size of the memory displacement in bytes. Valid values are 0, 1 (16/32/64-bit), 2 (16-bit), 4 (32-bit), 8 (64-bit).
	/// Note that the return value can be 1 and `memory_displacement()` may still not fit in
	/// a signed byte if it's an EVEX encoded instruction.
	/// Use this property if the operand has kind `OpKind::Memory`
	///
	/// # Arguments
	///
	/// * `new_value`: Displacement size
	pub fn set_memory_displ_size(&mut self, new_value: i32) {
		let enc_value = match new_value {
			0 => 0,
			1 => 1,
			2 => 2,
			4 => 3,
			_ => 4,
		};
		self.memory_flags = (((self.memory_flags as u32) & !(MemoryFlags::DISPL_SIZE_MASK << MemoryFlags::DISPL_SIZE_SHIFT))
			| (enc_value << MemoryFlags::DISPL_SIZE_SHIFT)) as u16;
	}

	#[inline]
	pub(crate) fn internal_set_memory_displ_size(&mut self, new_value: u16) {
		debug_assert!(new_value <= 4);
		self.memory_flags |= new_value << MemoryFlags::DISPL_SIZE_SHIFT;
	}

	/// `true` if the data is broadcasted (EVEX instructions only)
	#[inline]
	pub fn is_broadcast(&self) -> bool {
		(self.memory_flags & (MemoryFlags::BROADCAST as u16)) != 0
	}

	/// Sets the is broadcast flag (EVEX instructions only)
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_is_broadcast(&mut self, new_value: bool) {
		if new_value {
			self.memory_flags |= MemoryFlags::BROADCAST as u16;
		} else {
			self.memory_flags &= !(MemoryFlags::BROADCAST as u16);
		}
	}

	#[inline]
	pub(crate) fn internal_set_is_broadcast(&mut self) {
		self.memory_flags |= MemoryFlags::BROADCAST as u16;
	}

	/// Gets the size of the memory location that is referenced by the operand. See also `is_broadcast()`).
	/// Use this property if the operand has kind `OpKind::Memory`, `OpKind::Memory64`,
	/// `OpKind::MemorySegSI`, `OpKind::MemorySegESI`, `OpKind::MemorySegRSI`,
	/// `OpKind::MemoryESDI`, `OpKind::MemoryESEDI`, `OpKind::MemoryESRDI`
	#[inline]
	pub fn memory_size(&self) -> MemorySize {
		let mut index = self.code() as usize;
		if self.is_broadcast() {
			index += IcedConstants::NUMBER_OF_CODE_VALUES as usize;
		}
		// safe if code() is safe, which it is unless the user used unsafe code to write garbage values to it
		unsafe { mem::transmute(instructionmemorysizes::SIZES[index]) }
	}

	/// Gets the index register scale value, valid values are *1, *2, *4, *8. Use this property if the operand has kind `OpKind::Memory`
	#[inline]
	pub fn memory_index_scale(&self) -> i32 {
		1 << (self.memory_flags as u32 & MemoryFlags::SCALE_MASK)
	}

	/// Sets the index register scale value, valid values are *1, *2, *4, *8. Use this property if the operand has kind `OpKind::Memory`
	///
	/// # Arguments
	///
	/// * `new_value`: New value (1, 2, 4 or 8)
	pub fn set_memory_index_scale(&mut self, new_value: i32) {
		match new_value {
			1 => self.memory_flags &= !3,
			2 => self.memory_flags = (self.memory_flags & !(MemoryFlags::SCALE_MASK as u16)) | 1,
			4 => self.memory_flags = (self.memory_flags & !(MemoryFlags::SCALE_MASK as u16)) | 2,
			_ => {
				debug_assert!(new_value == 8);
				self.memory_flags |= 3;
			}
		}
	}

	#[inline]
	pub(crate) fn internal_get_memory_index_scale(&self) -> i32 {
		(self.memory_flags & (MemoryFlags::SCALE_MASK as u16)) as i32
	}

	#[inline]
	pub(crate) fn internal_set_memory_index_scale(&mut self, new_value: i32) {
		self.memory_flags |= new_value as u16;
	}

	/// Gets the memory operand's displacement. This should be sign extended to 64 bits if it's 64-bit addressing (see `memory_displacement64()`).
	/// Use this property if the operand has kind `OpKind::Memory`
	#[inline]
	pub fn memory_displacement(&self) -> u32 {
		self.mem_displ
	}

	/// Sets the memory operand's displacement. This should be sign extended to 64 bits if it's 64-bit addressing.
	/// Use this property if the operand has kind `OpKind::Memory`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_memory_displacement(&mut self, new_value: u32) {
		self.mem_displ = new_value;
	}

	/// Gets the memory operand's displacement sign extended to 64 bits.
	/// Use this property if the operand has kind `OpKind::Memory`
	#[inline]
	pub fn memory_displacement64(&self) -> u64 {
		self.mem_displ as i32 as u64
	}

	/// Gets an operand's immediate value
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	pub fn immediate(&self, operand: i32) -> u64 {
		match self.op_kind(operand) {
			OpKind::Immediate8 => self.immediate8() as u64,
			OpKind::Immediate8_2nd => self.immediate8_2nd() as u64,
			OpKind::Immediate16 => self.immediate16() as u64,
			OpKind::Immediate32 => self.immediate32() as u64,
			OpKind::Immediate64 => self.immediate64(),
			OpKind::Immediate8to16 => self.immediate8to16() as u64,
			OpKind::Immediate8to32 => self.immediate8to32() as u64,
			OpKind::Immediate8to64 => self.immediate8to64() as u64,
			OpKind::Immediate32to64 => self.immediate32to64() as u64,
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Sets an operand's immediate value
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	/// * `new_value`: Immediate
	#[inline]
	pub fn set_immediate_i32(&mut self, operand: i32, new_value: i32) {
		self.set_immediate_u64(operand, new_value as u64);
	}

	/// Sets an operand's immediate value
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	/// * `new_value`: Immediate
	#[inline]
	pub fn set_immediate_u32(&mut self, operand: i32, new_value: u32) {
		self.set_immediate_u64(operand, new_value as u64);
	}

	/// Sets an operand's immediate value
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	/// * `new_value`: Immediate
	#[inline]
	pub fn set_immediate_i64(&mut self, operand: i32, new_value: i64) {
		self.set_immediate_u64(operand, new_value as u64);
	}

	/// Sets an operand's immediate value
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	/// * `new_value`: Immediate
	pub fn set_immediate_u64(&mut self, operand: i32, new_value: u64) {
		match self.op_kind(operand) {
			OpKind::Immediate8 | OpKind::Immediate8to16 | OpKind::Immediate8to32 | OpKind::Immediate8to64 => self.immediate = new_value as u8 as u32,
			OpKind::Immediate8_2nd => self.mem_displ = new_value as u8 as u32,
			OpKind::Immediate16 => self.immediate = new_value as u16 as u32,
			OpKind::Immediate32to64 | OpKind::Immediate32 => self.immediate = new_value as u32,
			OpKind::Immediate64 => self.set_immediate64(new_value),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8`
	#[inline]
	pub fn immediate8(&self) -> u8 {
		self.immediate as u8
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate8(&mut self, new_value: u8) {
		self.immediate = new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_immediate8(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8_2nd`
	#[inline]
	pub fn immediate8_2nd(&self) -> u8 {
		self.mem_displ as u8
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8_2nd`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate8_2nd(&mut self, new_value: u8) {
		self.mem_displ = new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_immediate8_2nd(&mut self, new_value: u32) {
		self.mem_displ = new_value;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate16`
	#[inline]
	pub fn immediate16(&self) -> u16 {
		self.immediate as u16
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate16`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate16(&mut self, new_value: u16) {
		self.immediate = new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_immediate16(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate32`
	#[inline]
	pub fn immediate32(&self) -> u32 {
		self.immediate
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate32`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate32(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate64`
	#[inline]
	pub fn immediate64(&self) -> u64 {
		((self.mem_displ as u64) << 32) | (self.immediate as u64)
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate64`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate64(&mut self, new_value: u64) {
		self.immediate = new_value as u32;
		self.mem_displ = (new_value >> 32) as u32;
	}

	#[inline]
	pub(crate) fn internal_set_immediate64_lo(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	#[inline]
	pub(crate) fn internal_set_immediate64_hi(&mut self, new_value: u32) {
		self.mem_displ = new_value;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8to16`
	#[inline]
	pub fn immediate8to16(&self) -> i16 {
		self.immediate as i8 as i16
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8to16`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate8to16(&mut self, new_value: i16) {
		self.immediate = new_value as i8 as u32;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8to32`
	#[inline]
	pub fn immediate8to32(&self) -> i32 {
		self.immediate as i8 as i32
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8to32`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate8to32(&mut self, new_value: i32) {
		self.immediate = new_value as i8 as u32;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8to64`
	#[inline]
	pub fn immediate8to64(&self) -> i64 {
		self.immediate as i8 as i64
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate8to64`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate8to64(&mut self, new_value: i64) {
		self.immediate = new_value as i8 as u32;
	}

	/// Gets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate32to64`
	#[inline]
	pub fn immediate32to64(&self) -> i64 {
		self.immediate as i32 as i64
	}

	/// Sets the operand's immediate value. Use this property if the operand has kind `OpKind::Immediate32to64`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_immediate32to64(&mut self, new_value: i64) {
		self.immediate = new_value as u32;
	}

	/// Gets the operand's 64-bit address value. Use this property if the operand has kind `OpKind::Memory64`
	#[inline]
	pub fn memory_address64(&self) -> u64 {
		((self.mem_displ as u64) << 32) | self.immediate as u64
	}

	/// Sets the operand's 64-bit address value. Use this property if the operand has kind `OpKind::Memory64`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_memory_address64(&mut self, new_value: u64) {
		self.immediate = new_value as u32;
		self.mem_displ = (new_value >> 32) as u32;
	}

	#[inline]
	pub(crate) fn internal_set_memory_address64_lo(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	#[inline]
	pub(crate) fn internal_set_memory_address64_hi(&mut self, new_value: u32) {
		self.mem_displ = new_value;
	}

	/// Gets the operand's branch target. Use this property if the operand has kind `OpKind::NearBranch16`
	#[inline]
	pub fn near_branch16(&self) -> u16 {
		self.immediate as u16
	}

	/// Sets the operand's branch target. Use this property if the operand has kind `OpKind::NearBranch16`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_near_branch16(&mut self, new_value: u16) {
		self.immediate = new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_near_branch16(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's branch target. Use this property if the operand has kind `OpKind::NearBranch32`
	#[inline]
	pub fn near_branch32(&self) -> u32 {
		self.immediate
	}

	/// Sets the operand's branch target. Use this property if the operand has kind `OpKind::NearBranch32`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_near_branch32(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's branch target. Use this property if the operand has kind `OpKind::NearBranch64`
	#[inline]
	pub fn near_branch64(&self) -> u64 {
		((self.mem_displ as u64) << 32) | self.immediate as u64
	}

	/// Sets the operand's branch target. Use this property if the operand has kind `OpKind::NearBranch64`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_near_branch64(&mut self, new_value: u64) {
		self.immediate = new_value as u32;
		self.mem_displ = (new_value >> 32) as u32;
	}

	/// Gets the near branch target if it's a call/jmp near branch instruction
	pub fn near_branch_target(&self) -> u64 {
		match self.op0_kind() {
			OpKind::NearBranch16 => self.near_branch16() as u64,
			OpKind::NearBranch32 => self.near_branch32() as u64,
			OpKind::NearBranch64 => self.near_branch64(),
			_ => 0,
		}
	}

	/// Gets the operand's branch target. Use this property if the operand has kind `OpKind::FarBranch16`
	#[inline]
	pub fn far_branch16(&self) -> u16 {
		self.immediate as u16
	}

	/// Sets the operand's branch target. Use this property if the operand has kind `OpKind::FarBranch16`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_far_branch16(&mut self, new_value: u16) {
		self.immediate = new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_far_branch16(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's branch target. Use this property if the operand has kind `OpKind::FarBranch32`
	#[inline]
	pub fn far_branch32(&self) -> u32 {
		self.immediate
	}

	/// Sets the operand's branch target. Use this property if the operand has kind `OpKind::FarBranch32`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_far_branch32(&mut self, new_value: u32) {
		self.immediate = new_value;
	}

	/// Gets the operand's branch target selector. Use this property if the operand has kind `OpKind::FarBranch16` or `OpKind::FarBranch32`
	#[inline]
	pub fn far_branch_selector(&self) -> u16 {
		self.mem_displ as u16
	}

	/// Sets the operand's branch target selector. Use this property if the operand has kind `OpKind::FarBranch16` or `OpKind::FarBranch32`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_far_branch_selector(&mut self, new_value: u16) {
		self.mem_displ = new_value as u32;
	}

	#[inline]
	pub(crate) fn internal_set_far_branch_selector(&mut self, new_value: u32) {
		self.mem_displ = new_value;
	}

	/// Gets the memory operand's base register or `Register::None` if none. Use this property if the operand has kind `OpKind::Memory`
	#[inline]
	pub fn memory_base(&self) -> Register {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.mem_base_reg) }
	}

	/// Sets the memory operand's base register or `Register::None` if none. Use this property if the operand has kind `OpKind::Memory`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_memory_base(&mut self, new_value: Register) {
		self.mem_base_reg = new_value as u8;
	}

	#[inline]
	pub(crate) fn internal_set_memory_base(&mut self, new_value: Register) {
		self.mem_base_reg = new_value as u8;
	}

	/// Gets the memory operand's index register or `Register::None` if none. Use this property if the operand has kind `OpKind::Memory`
	#[inline]
	pub fn memory_index(&self) -> Register {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.mem_index_reg) }
	}

	/// Sets the memory operand's index register or `Register::None` if none. Use this property if the operand has kind `OpKind::Memory`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_memory_index(&mut self, new_value: Register) {
		self.mem_index_reg = new_value as u8;
	}

	#[inline]
	pub(crate) fn internal_set_memory_index(&mut self, new_value: Register) {
		self.mem_index_reg = new_value as u8;
	}

	/// Gets operand #0's register value. Use this property if operand #0 (`op0_kind()`) has kind `OpKind::Register`
	#[inline]
	pub fn op0_register(&self) -> Register {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.reg0) }
	}

	/// Sets operand #0's register value. Use this property if operand #0 (`op0_kind()`) has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_op0_register(&mut self, new_value: Register) {
		self.reg0 = new_value as u8;
	}

	#[inline]
	pub(crate) fn internal_set_op0_register(&mut self, new_value: Register) {
		self.reg0 = new_value as u8;
	}

	/// Gets operand #1's register value. Use this property if operand #1 (`op1_kind()`) has kind `OpKind::Register`
	#[inline]
	pub fn op1_register(&self) -> Register {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.reg1) }
	}

	/// Sets operand #1's register value. Use this property if operand #1 (`op1_kind()`) has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_op1_register(&mut self, new_value: Register) {
		self.reg1 = new_value as u8;
	}

	#[inline]
	pub(crate) fn internal_set_op1_register(&mut self, new_value: Register) {
		self.reg1 = new_value as u8;
	}

	/// Gets operand #2's register value. Use this property if operand #2 (`op2_kind()`) has kind `OpKind::Register`
	#[inline]
	pub fn op2_register(&self) -> Register {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.reg2) }
	}

	/// Sets operand #2's register value. Use this property if operand #2 (`op2_kind()`) has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_op2_register(&mut self, new_value: Register) {
		self.reg2 = new_value as u8;
	}

	#[inline]
	pub(crate) fn internal_set_op2_register(&mut self, new_value: Register) {
		self.reg2 = new_value as u8;
	}

	/// Gets operand #3's register value. Use this property if operand #3 (`op3_kind()`) has kind `OpKind::Register`
	#[inline]
	pub fn op3_register(&self) -> Register {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(self.reg3) }
	}

	/// Sets operand #3's register value. Use this property if operand #3 (`op3_kind()`) has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_op3_register(&mut self, new_value: Register) {
		self.reg3 = new_value as u8;
	}

	#[inline]
	pub(crate) fn internal_set_op3_register(&mut self, new_value: Register) {
		self.reg3 = new_value as u8;
	}

	/// Gets operand #4's register value. Use this property if operand #4 (`op4_kind()`) has kind `OpKind::Register`
	#[inline]
	pub fn op4_register(&self) -> Register {
		Register::None
	}

	/// Sets operand #4's register value. Use this property if operand #4 (`op4_kind()`) has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_op4_register(&mut self, new_value: Register) {
		if new_value != Register::None {
			panic!("NYI"); //TODO:
		}
	}

	/// Gets the operand's register value. Use this property if the operand has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	pub fn op_register(&self, operand: i32) -> Register {
		match operand {
			0 => self.op0_register(),
			1 => self.op1_register(),
			2 => self.op2_register(),
			3 => self.op3_register(),
			4 => self.op4_register(),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Sets the operand's register value. Use this property if the operand has kind `OpKind::Register`
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, 0-4
	/// * `new_value`: New value
	pub fn set_op_register(&mut self, operand: i32, new_value: Register) {
		match operand {
			0 => self.set_op0_register(new_value),
			1 => self.set_op1_register(new_value),
			2 => self.set_op2_register(new_value),
			3 => self.set_op3_register(new_value),
			4 => self.set_op4_register(new_value),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets the opmask register (`Register::K1` - `Register::K7`) or `Register::None` if none
	#[inline]
	pub fn opmask(&self) -> Register {
		let r = (self.code_flags >> CodeFlags::OPMASK_SHIFT) & CodeFlags::OPMASK_MASK;
		if r == 0 {
			Register::None
		} else {
			// safe, it's guaranteed to return K1-K7 since we only store 3 register bits (r == 1..7)
			unsafe { mem::transmute((r + Register::K0 as u32) as u8) }
		}
	}

	/// Sets the opmask register (`Register::K1` - `Register::K7`) or `Register::None` if none
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	pub fn set_opmask(&mut self, new_value: Register) {
		let r = if new_value == Register::None {
			0
		} else {
			(new_value as u32 - Register::K0 as u32) & CodeFlags::OPMASK_MASK
		};
		self.code_flags = (self.code_flags & !(CodeFlags::OPMASK_MASK << CodeFlags::OPMASK_SHIFT)) | (r << CodeFlags::OPMASK_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_opmask(&self) -> u32 {
		(self.code_flags >> CodeFlags::OPMASK_SHIFT) & CodeFlags::OPMASK_MASK
	}

	#[inline]
	pub(crate) fn internal_set_opmask(&mut self, new_value: u32) {
		self.code_flags |= new_value << CodeFlags::OPMASK_SHIFT
	}

	/// Checks if there's an opmask register (`opmask()`)
	#[inline]
	pub fn has_opmask(&self) -> bool {
		(self.code_flags & (CodeFlags::OPMASK_MASK << CodeFlags::OPMASK_SHIFT)) != 0
	}

	/// `true` if zeroing-masking, `false` if merging-masking.
	/// Only used by most EVEX encoded instructions that use opmask registers.
	#[inline]
	pub fn zeroing_masking(&self) -> bool {
		(self.code_flags & CodeFlags::ZEROING_MASKING) != 0
	}

	/// `true` if zeroing-masking, `false` if merging-masking.
	/// Only used by most EVEX encoded instructions that use opmask registers.
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_zeroing_masking(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::ZEROING_MASKING;
		} else {
			self.code_flags &= !CodeFlags::ZEROING_MASKING;
		}
	}

	#[inline]
	pub(crate) fn internal_set_zeroing_masking(&mut self) {
		self.code_flags |= CodeFlags::ZEROING_MASKING;
	}

	/// `true` if merging-masking, `false` if zeroing-masking.
	/// Only used by most EVEX encoded instructions that use opmask registers.
	#[inline]
	pub fn merging_masking(&self) -> bool {
		(self.code_flags & CodeFlags::ZEROING_MASKING) == 0
	}

	/// `true` if merging-masking, `false` if zeroing-masking.
	/// Only used by most EVEX encoded instructions that use opmask registers.
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_merging_masking(&mut self, new_value: bool) {
		if new_value {
			self.code_flags &= !CodeFlags::ZEROING_MASKING;
		} else {
			self.code_flags |= CodeFlags::ZEROING_MASKING;
		}
	}

	/// Gets the rounding control (`suppress_all_exceptions()` is implied but still returns `false`)
	/// or `RoundingControl::None` if the instruction doesn't use it.
	#[inline]
	pub fn rounding_control(&self) -> RoundingControl {
		// safe: iced only initializes the bits to valid values. The user can write garbage if he/she uses unsafe code though.
		unsafe { mem::transmute(((self.code_flags >> CodeFlags::ROUNDING_CONTROL_SHIFT) & CodeFlags::ROUNDING_CONTROL_MASK) as u8) }
	}

	/// Sets the rounding control (`suppress_all_exceptions()` is implied but still returns `false`)
	/// or `RoundingControl::None` if the instruction doesn't use it.
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_rounding_control(&mut self, new_value: RoundingControl) {
		self.code_flags = (self.code_flags & !(CodeFlags::ROUNDING_CONTROL_MASK << CodeFlags::ROUNDING_CONTROL_SHIFT))
			| ((new_value as u32 & CodeFlags::ROUNDING_CONTROL_MASK) << CodeFlags::ROUNDING_CONTROL_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_rounding_control(&mut self, new_value: u32) {
		self.code_flags |= new_value << CodeFlags::ROUNDING_CONTROL_SHIFT;
	}

	/// Gets the number of elements in a `db`/`dw`/`dd`/`dq` directive.
	/// Can only be called if `code()` is `Code::DeclareByte`, `Code::DeclareWord`, `Code::DeclareDword`, `Code::DeclareQword`
	#[inline]
	pub fn declare_data_len(&self) -> i32 {
		((self.op_kind_flags >> OpKindFlags::DATA_LENGTH_SHIFT) & OpKindFlags::DATA_LENGTH_MASK) as i32 + 1
	}

	/// Sets the number of elements in a `db`/`dw`/`dd`/`dq` directive.
	/// Can only be called if `code()` is `Code::DeclareByte`, `Code::DeclareWord`, `Code::DeclareDword`, `Code::DeclareQword`
	///
	/// # Arguments
	///
	/// * `new_value`: New value: `db`: 1-16; `dw`: 1-8; `dd`: 1-4; `dq`: 1-2
	#[inline]
	pub fn set_declare_data_len(&mut self, new_value: i32) {
		self.op_kind_flags = (self.op_kind_flags & !(OpKindFlags::DATA_LENGTH_MASK << OpKindFlags::DATA_LENGTH_SHIFT))
			| (((new_value - 1) as u32 & OpKindFlags::DATA_LENGTH_MASK) << OpKindFlags::DATA_LENGTH_SHIFT);
	}

	#[inline]
	pub(crate) fn internal_set_declare_data_len(&mut self, new_value: u32) {
		self.op_kind_flags |= (new_value - 1) << OpKindFlags::DATA_LENGTH_SHIFT;
	}

	/// Sets a new `db` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareByte`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-15)
	/// * `new_value`: New value
	#[inline]
	pub fn set_declare_byte_value_i8(&mut self, index: i32, new_value: i8) {
		self.set_declare_byte_value(index, new_value as u8)
	}

	/// Sets a new `db` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareByte`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-15)
	/// * `new_value`: New value
	pub fn set_declare_byte_value(&mut self, index: i32, new_value: u8) {
		match index {
			0 => self.reg0 = new_value,
			1 => self.reg1 = new_value,
			2 => self.reg2 = new_value,
			3 => self.reg3 = new_value,
			4 => self.immediate = (self.immediate & 0xFFFF_FF00) | new_value as u32,
			5 => self.immediate = (self.immediate & 0xFFFF_00FF) | ((new_value as u32) << 8),
			6 => self.immediate = (self.immediate & 0xFF00_FFFF) | ((new_value as u32) << 16),
			7 => self.immediate = (self.immediate & 0x00FF_FFFF) | ((new_value as u32) << 24),
			8 => self.mem_displ = (self.mem_displ & 0xFFFF_FF00) | new_value as u32,
			9 => self.mem_displ = (self.mem_displ & 0xFFFF_00FF) | ((new_value as u32) << 8),
			10 => self.mem_displ = (self.mem_displ & 0xFF00_FFFF) | ((new_value as u32) << 16),
			11 => self.mem_displ = (self.mem_displ & 0x00FF_FFFF) | ((new_value as u32) << 24),
			12 => self.mem_base_reg = new_value,
			13 => self.mem_index_reg = new_value,
			14 => self.op_kind_flags = (self.op_kind_flags & 0xFFFF_FF00) | new_value as u32,
			15 => self.op_kind_flags = (self.op_kind_flags & 0xFFFF_00FF) | ((new_value as u32) << 8),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets a `db` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareByte`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-15)
	pub fn get_declare_byte_value(&self, index: i32) -> u8 {
		match index {
			0 => self.reg0,
			1 => self.reg1,
			2 => self.reg2,
			3 => self.reg3,
			4 => self.immediate as u8,
			5 => (self.immediate >> 8) as u8,
			6 => (self.immediate >> 16) as u8,
			7 => (self.immediate >> 24) as u8,
			8 => self.mem_displ as u8,
			9 => (self.mem_displ >> 8) as u8,
			10 => (self.mem_displ >> 16) as u8,
			11 => (self.mem_displ >> 24) as u8,
			12 => self.mem_base_reg,
			13 => self.mem_index_reg,
			14 => self.op_kind_flags as u8,
			15 => (self.op_kind_flags >> 8) as u8,
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Sets a new `dw` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareWord`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-7)
	/// * `new_value`: New value
	#[inline]
	pub fn set_declare_word_value_i16(&mut self, index: i32, new_value: i16) {
		self.set_declare_word_value(index, new_value as u16);
	}

	/// Sets a new `dw` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareWord`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-7)
	/// * `new_value`: New value
	pub fn set_declare_word_value(&mut self, index: i32, new_value: u16) {
		match index {
			0 => {
				self.reg0 = new_value as u8;
				self.reg1 = (new_value >> 8) as u8;
			}
			1 => {
				self.reg2 = new_value as u8;
				self.reg3 = (new_value >> 8) as u8;
			}
			2 => self.immediate = (self.immediate & 0xFFFF_0000) | new_value as u32,
			3 => self.immediate = self.immediate as u16 as u32 | (new_value as u32) << 16,
			4 => self.mem_displ = (self.mem_displ & 0xFFFF_0000) | new_value as u32,
			5 => self.mem_displ = self.mem_displ as u16 as u32 | (new_value as u32) << 16,
			6 => {
				self.mem_base_reg = new_value as u8;
				self.mem_index_reg = (new_value >> 8) as u8;
			}
			7 => self.op_kind_flags = (self.op_kind_flags & 0xFFFF_0000) | new_value as u32,
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets a `dw` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareWord`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-7)
	pub fn get_declare_word_value(&self, index: i32) -> u16 {
		match index {
			0 => self.reg0 as u16 | ((self.reg1 as u16) << 8),
			1 => self.reg2 as u16 | ((self.reg3 as u16) << 8),
			2 => self.immediate as u16,
			3 => (self.immediate >> 16) as u16,
			4 => self.mem_displ as u16,
			5 => (self.mem_displ >> 16) as u16,
			6 => self.mem_base_reg as u16 | ((self.mem_index_reg as u16) << 8),
			7 => self.op_kind_flags as u16,
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Sets a new `dd` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareDword`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-3)
	/// * `new_value`: New value
	#[inline]
	pub fn set_declare_dword_value_i32(&mut self, index: i32, new_value: i32) {
		self.set_declare_dword_value(index, new_value as u32);
	}

	/// Sets a new `dd` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareDword`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-3)
	/// * `new_value`: New value
	pub fn set_declare_dword_value(&mut self, index: i32, new_value: u32) {
		match index {
			0 => {
				self.reg0 = new_value as u8;
				self.reg1 = (new_value >> 8) as u8;
				self.reg2 = (new_value >> 16) as u8;
				self.reg3 = (new_value >> 24) as u8;
			}
			1 => self.immediate = new_value,
			2 => self.mem_displ = new_value,
			3 => {
				self.mem_base_reg = new_value as u8;
				self.mem_index_reg = (new_value >> 8) as u8;
				self.op_kind_flags = (self.op_kind_flags & 0xFFFF_0000) | (new_value >> 16);
			}
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets a `dd` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareDword`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-3)
	pub fn get_declare_dword_value(&self, index: i32) -> u32 {
		match index {
			0 => self.reg0 as u32 | ((self.reg1 as u32) << 8) | ((self.reg2 as u32) << 16) | ((self.reg3 as u32) << 24),
			1 => self.immediate,
			2 => self.mem_displ,
			3 => self.mem_base_reg as u32 | ((self.mem_index_reg as u32) << 8) | (self.op_kind_flags << 16),
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Sets a new `dq` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareQword`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-1)
	/// * `new_value`: New value
	#[inline]
	pub fn set_declare_qword_value_i64(&mut self, index: i32, new_value: i64) {
		self.set_declare_qword_value(index, new_value as u64);
	}

	/// Sets a new `dq` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareQword`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-1)
	/// * `new_value`: New value
	pub fn set_declare_qword_value(&mut self, index: i32, new_value: u64) {
		match index {
			0 => {
				self.reg0 = new_value as u8;
				self.reg1 = (new_value >> 8) as u8;
				self.reg2 = (new_value >> 16) as u8;
				self.reg3 = (new_value >> 24) as u8;
				self.immediate = (new_value >> 32) as u32;
			}
			1 => {
				self.mem_displ = new_value as u32;
				self.mem_base_reg = (new_value >> 32) as u8;
				self.mem_index_reg = (new_value >> 40) as u8;
				self.op_kind_flags = (self.op_kind_flags & 0xFFFF_0000) | (new_value >> 48) as u32;
			}
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Gets a `dq` value, see also `declare_data_len()`.
	/// Can only be called if `code()` is `Code::DeclareQword`
	///
	/// # Arguments
	///
	/// * `index`: Index (0-1)
	pub fn get_declare_qword_value(&self, index: i32) -> u64 {
		match index {
			0 => {
				self.reg0 as u64
					| ((self.reg1 as u64) << 8)
					| ((self.reg2 as u64) << 16)
					| ((self.reg3 as u64) << 24)
					| ((self.immediate as u64) << 32)
			}
			1 => {
				self.mem_displ as u64 | ((self.mem_base_reg as u64) << 32) | ((self.mem_index_reg as u64) << 40) | ((self.op_kind_flags as u64) << 48)
			}
			_ => panic!("NYI"), //TODO:
		}
	}

	/// Checks if this is a VSIB instruction, see also `is_vsib32()`, `is_vsib64()`
	#[inline]
	pub fn is_vsib(&self) -> bool {
		self.vsib().is_some()
	}

	/// VSIB instructions only (`is_vsib()`): `true` if it's using 32-bit indexes, `false` if it's using 64-bit indexes
	#[inline]
	pub fn is_vsib32(&self) -> bool {
		if let Some(is_vsib64) = self.vsib() {
			!is_vsib64
		} else {
			false
		}
	}

	/// VSIB instructions only (`is_vsib()`): `true` if it's using 64-bit indexes, `false` if it's using 32-bit indexes
	#[inline]
	pub fn is_vsib64(&self) -> bool {
		if let Some(is_vsib64) = self.vsib() {
			is_vsib64
		} else {
			false
		}
	}

	/// Checks if it's a vsib instruction.
	/// # Returns
	///
	/// * `Some(true)` if it's a VSIB instruction with 64-bit indexes
	/// * `Some(false)` if it's a VSIB instruction with 32-bit indexes
	/// * `None` if it's not a VSIB instruction.
	pub fn vsib(&self) -> Option<bool> {
		match self.code() {
			Code::VEX_Vpgatherdd_xmm_vm32x_xmm
			| Code::VEX_Vpgatherdd_ymm_vm32y_ymm
			| Code::VEX_Vpgatherdq_xmm_vm32x_xmm
			| Code::VEX_Vpgatherdq_ymm_vm32x_ymm
			| Code::EVEX_Vpgatherdd_xmm_k1_vm32x
			| Code::EVEX_Vpgatherdd_ymm_k1_vm32y
			| Code::EVEX_Vpgatherdd_zmm_k1_vm32z
			| Code::EVEX_Vpgatherdq_xmm_k1_vm32x
			| Code::EVEX_Vpgatherdq_ymm_k1_vm32x
			| Code::EVEX_Vpgatherdq_zmm_k1_vm32y
			| Code::VEX_Vgatherdps_xmm_vm32x_xmm
			| Code::VEX_Vgatherdps_ymm_vm32y_ymm
			| Code::VEX_Vgatherdpd_xmm_vm32x_xmm
			| Code::VEX_Vgatherdpd_ymm_vm32x_ymm
			| Code::EVEX_Vgatherdps_xmm_k1_vm32x
			| Code::EVEX_Vgatherdps_ymm_k1_vm32y
			| Code::EVEX_Vgatherdps_zmm_k1_vm32z
			| Code::EVEX_Vgatherdpd_xmm_k1_vm32x
			| Code::EVEX_Vgatherdpd_ymm_k1_vm32x
			| Code::EVEX_Vgatherdpd_zmm_k1_vm32y
			| Code::EVEX_Vpscatterdd_vm32x_k1_xmm
			| Code::EVEX_Vpscatterdd_vm32y_k1_ymm
			| Code::EVEX_Vpscatterdd_vm32z_k1_zmm
			| Code::EVEX_Vpscatterdq_vm32x_k1_xmm
			| Code::EVEX_Vpscatterdq_vm32x_k1_ymm
			| Code::EVEX_Vpscatterdq_vm32y_k1_zmm
			| Code::EVEX_Vscatterdps_vm32x_k1_xmm
			| Code::EVEX_Vscatterdps_vm32y_k1_ymm
			| Code::EVEX_Vscatterdps_vm32z_k1_zmm
			| Code::EVEX_Vscatterdpd_vm32x_k1_xmm
			| Code::EVEX_Vscatterdpd_vm32x_k1_ymm
			| Code::EVEX_Vscatterdpd_vm32y_k1_zmm
			| Code::EVEX_Vgatherpf0dps_vm32z_k1
			| Code::EVEX_Vgatherpf0dpd_vm32y_k1
			| Code::EVEX_Vgatherpf1dps_vm32z_k1
			| Code::EVEX_Vgatherpf1dpd_vm32y_k1
			| Code::EVEX_Vscatterpf0dps_vm32z_k1
			| Code::EVEX_Vscatterpf0dpd_vm32y_k1
			| Code::EVEX_Vscatterpf1dps_vm32z_k1
			| Code::EVEX_Vscatterpf1dpd_vm32y_k1 => Some(false),

			Code::VEX_Vpgatherqd_xmm_vm64x_xmm
			| Code::VEX_Vpgatherqd_xmm_vm64y_xmm
			| Code::VEX_Vpgatherqq_xmm_vm64x_xmm
			| Code::VEX_Vpgatherqq_ymm_vm64y_ymm
			| Code::EVEX_Vpgatherqd_xmm_k1_vm64x
			| Code::EVEX_Vpgatherqd_xmm_k1_vm64y
			| Code::EVEX_Vpgatherqd_ymm_k1_vm64z
			| Code::EVEX_Vpgatherqq_xmm_k1_vm64x
			| Code::EVEX_Vpgatherqq_ymm_k1_vm64y
			| Code::EVEX_Vpgatherqq_zmm_k1_vm64z
			| Code::VEX_Vgatherqps_xmm_vm64x_xmm
			| Code::VEX_Vgatherqps_xmm_vm64y_xmm
			| Code::VEX_Vgatherqpd_xmm_vm64x_xmm
			| Code::VEX_Vgatherqpd_ymm_vm64y_ymm
			| Code::EVEX_Vgatherqps_xmm_k1_vm64x
			| Code::EVEX_Vgatherqps_xmm_k1_vm64y
			| Code::EVEX_Vgatherqps_ymm_k1_vm64z
			| Code::EVEX_Vgatherqpd_xmm_k1_vm64x
			| Code::EVEX_Vgatherqpd_ymm_k1_vm64y
			| Code::EVEX_Vgatherqpd_zmm_k1_vm64z
			| Code::EVEX_Vpscatterqd_vm64x_k1_xmm
			| Code::EVEX_Vpscatterqd_vm64y_k1_xmm
			| Code::EVEX_Vpscatterqd_vm64z_k1_ymm
			| Code::EVEX_Vpscatterqq_vm64x_k1_xmm
			| Code::EVEX_Vpscatterqq_vm64y_k1_ymm
			| Code::EVEX_Vpscatterqq_vm64z_k1_zmm
			| Code::EVEX_Vscatterqps_vm64x_k1_xmm
			| Code::EVEX_Vscatterqps_vm64y_k1_xmm
			| Code::EVEX_Vscatterqps_vm64z_k1_ymm
			| Code::EVEX_Vscatterqpd_vm64x_k1_xmm
			| Code::EVEX_Vscatterqpd_vm64y_k1_ymm
			| Code::EVEX_Vscatterqpd_vm64z_k1_zmm
			| Code::EVEX_Vgatherpf0qps_vm64z_k1
			| Code::EVEX_Vgatherpf0qpd_vm64z_k1
			| Code::EVEX_Vgatherpf1qps_vm64z_k1
			| Code::EVEX_Vgatherpf1qpd_vm64z_k1
			| Code::EVEX_Vscatterpf0qps_vm64z_k1
			| Code::EVEX_Vscatterpf0qpd_vm64z_k1
			| Code::EVEX_Vscatterpf1qps_vm64z_k1
			| Code::EVEX_Vscatterpf1qpd_vm64z_k1 => Some(true),

			_ => None,
		}
	}

	/// Gets the suppress all exceptions flag (EVEX encoded instructions). Note that if `rounding_control()` is
	/// not `RoundingControl::None`, SAE is implied but this property will still return `false`.
	#[inline]
	pub fn suppress_all_exceptions(&self) -> bool {
		(self.code_flags & CodeFlags::SUPPRESS_ALL_EXCEPTIONS) != 0
	}

	/// Sets the suppress all exceptions flag (EVEX encoded instructions). Note that if `rounding_control()` is
	/// not `RoundingControl::None`, SAE is implied but this property will still return `false`.
	///
	/// # Arguments
	///
	/// * `new_value`: New value
	#[inline]
	pub fn set_suppress_all_exceptions(&mut self, new_value: bool) {
		if new_value {
			self.code_flags |= CodeFlags::SUPPRESS_ALL_EXCEPTIONS;
		} else {
			self.code_flags &= !CodeFlags::SUPPRESS_ALL_EXCEPTIONS;
		}
	}

	#[inline]
	pub(crate) fn internal_set_suppress_all_exceptions(&mut self) {
		self.code_flags |= CodeFlags::SUPPRESS_ALL_EXCEPTIONS;
	}

	/// Checks if the memory operand is `RIP`/`EIP` relative
	#[inline]
	pub fn is_ip_relative_memory_operand(&self) -> bool {
		let base_reg = self.memory_base();
		base_reg == Register::RIP || base_reg == Register::EIP
	}

	/// Gets the `RIP`/`EIP` releative address ((`next_ip()` or `next_ip32()`) + `memory_displacement()`).
	/// This property is only valid if there's a memory operand with `RIP`/`EIP` relative addressing, see `is_ip_relative_memory_operand()`
	pub fn ip_relative_memory_address(&self) -> u64 {
		let mut result = self.next_ip() + self.memory_displacement() as i32 as u64;
		if self.memory_base() == Register::EIP {
			result = result as u32 as u64;
		}
		result
	}

	/// Gets the virtual address of a memory operand
	///
	/// # Arguments
	///
	/// * `operand`: Operand number, must be a memory operand
	/// * `element_index`: Only used if it's a vsib memory operand. This is the element index of the vector index register.
	/// * `get_register_value`: Function that returns the value of a register or the base address of a segment register.
	///
	/// # Call-back function args
	///
	/// * Arg 1: `register`: Register (GPR8, GPR16, GPR32, GPR64, XMM, YMM, ZMM, seg). If it's a segment register, the call-back function should return the segment's base value, not the segment register value.
	/// * Arg 2: `element_index`: Only used if it's a vsib memory operand. This is the element index in the vector register.
	/// * Arg 3: `element_size`: Only used if it's a vsib memory operand. Size in bytes of elements in vector index register (4 or 8).
	pub fn virtual_address<F>(&self, operand: i32, element_index: i32, get_register_value: F) -> u64
	where
		F: Fn(Register, i32, i32) -> u64,
	{
		match self.op_kind(operand) {
			OpKind::Register
			| OpKind::NearBranch16
			| OpKind::NearBranch32
			| OpKind::NearBranch64
			| OpKind::FarBranch16
			| OpKind::FarBranch32
			| OpKind::Immediate8
			| OpKind::Immediate8_2nd
			| OpKind::Immediate16
			| OpKind::Immediate32
			| OpKind::Immediate64
			| OpKind::Immediate8to16
			| OpKind::Immediate8to32
			| OpKind::Immediate8to64
			| OpKind::Immediate32to64 => 0,

			OpKind::MemorySegSI => get_register_value(self.memory_segment(), 0, 0) + get_register_value(Register::SI, 0, 0) as u16 as u64,
			OpKind::MemorySegESI => get_register_value(self.memory_segment(), 0, 0) + get_register_value(Register::ESI, 0, 0) as u32 as u64,
			OpKind::MemorySegRSI => get_register_value(self.memory_segment(), 0, 0) + get_register_value(Register::RSI, 0, 0),
			OpKind::MemorySegDI => get_register_value(self.memory_segment(), 0, 0) + get_register_value(Register::DI, 0, 0) as u16 as u64,
			OpKind::MemorySegEDI => get_register_value(self.memory_segment(), 0, 0) + get_register_value(Register::EDI, 0, 0) as u32 as u64,
			OpKind::MemorySegRDI => get_register_value(self.memory_segment(), 0, 0) + get_register_value(Register::RDI, 0, 0),
			OpKind::MemoryESDI => get_register_value(Register::ES, 0, 0) + get_register_value(Register::DI, 0, 0) as u16 as u64,
			OpKind::MemoryESEDI => get_register_value(Register::ES, 0, 0) + get_register_value(Register::EDI, 0, 0) as u32 as u64,
			OpKind::MemoryESRDI => get_register_value(Register::ES, 0, 0) + get_register_value(Register::RDI, 0, 0),
			OpKind::Memory64 => get_register_value(self.memory_segment(), 0, 0) + self.memory_address64(),

			OpKind::Memory => {
				let base_reg = self.memory_base();
				let index_reg = self.memory_index();
				let addr_size = Self::get_address_size_in_bytes(base_reg, index_reg, self.memory_displ_size(), self.code_size());
				let mut offset = self.memory_displacement() as u64;
				let offset_mask = match addr_size {
					8 => {
						offset = offset as i32 as u64;
						std::u64::MAX
					}
					4 => std::u32::MAX as u64,
					_ => {
						debug_assert!(addr_size == 2);
						std::u16::MAX as u64
					}
				};
				match base_reg {
					Register::None => {}
					Register::RIP => offset += self.next_ip(),
					Register::EIP => offset += self.next_ip32() as u64,
					_ => offset += get_register_value(base_reg, 0, 0),
				}
				if index_reg != Register::None {
					if let Some(is_vsib64) = self.vsib() {
						if is_vsib64 {
							offset += get_register_value(index_reg, element_index, 8) << self.internal_get_memory_index_scale();
						} else {
							offset += (get_register_value(index_reg, element_index, 4) as u32 as u64) << self.internal_get_memory_index_scale();
						}
					} else {
						offset += get_register_value(index_reg, element_index, 0) << self.internal_get_memory_index_scale();
					}
				}
				offset &= offset_mask;
				get_register_value(self.memory_segment(), 0, 0) + offset
			}
		}
	}

	pub(crate) fn get_address_size_in_bytes(base_reg: Register, index_reg: Register, displ_size: i32, code_size: CodeSize) -> i32 {
		if (Register::RAX <= base_reg && base_reg <= Register::R15)
			|| (Register::RAX <= index_reg && index_reg <= Register::R15)
			|| base_reg == Register::RIP
		{
			return 8;
		}
		if (Register::EAX <= base_reg && base_reg <= Register::R15D)
			|| (Register::EAX <= index_reg && index_reg <= Register::R15D)
			|| base_reg == Register::EIP
		{
			return 4;
		}
		if base_reg == Register::BX
			|| base_reg == Register::BP
			|| base_reg == Register::SI
			|| base_reg == Register::DI
			|| index_reg == Register::SI
			|| index_reg == Register::DI
		{
			return 2;
		}
		if displ_size == 2 || displ_size == 4 || displ_size == 8 {
			return displ_size;
		}

		match code_size {
			CodeSize::Code64 => 8,
			CodeSize::Code32 => 4,
			CodeSize::Code16 => 2,
			_ => 8,
		}
	}
}

impl Instruction {
	//TODO: OpCode prop
}

#[cfg(feature = "ENCODER")]
impl Instruction {
	//TODO: Instruction.Create.cs
}

#[cfg(feature = "INSTR_INFO")]
impl Instruction {
	//TODO: Instruction.Info.cs
}

impl Eq for Instruction {}

impl PartialEq<Instruction> for Instruction {
	fn eq(&self, other: &Instruction) -> bool {
		((self.code_flags ^ other.code_flags) & !CodeFlags::EQUALS_IGNORE_MASK) == 0
			&& ((self.op_kind_flags ^ other.op_kind_flags) & !OpKindFlags::EQUALS_IGNORE_MASK) == 0
			&& self.immediate == other.immediate
			&& self.mem_displ == other.mem_displ
			&& self.memory_flags == other.memory_flags
			&& self.mem_base_reg == other.mem_base_reg
			&& self.mem_index_reg == other.mem_index_reg
			&& self.reg0 == other.reg0
			&& self.reg1 == other.reg1
			&& self.reg2 == other.reg2
			&& self.reg3 == other.reg3
	}
}

#[cfg(any(feature = "MASM_FORMATTER", feature = "ALL_FORMATTERS"))]
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("NYI")?; //TODO:
		Ok(())
	}
}
#[cfg(all(
	not(any(feature = "MASM_FORMATTER", feature = "ALL_FORMATTERS")),
	any(feature = "NASM_FORMATTER", feature = "ALL_FORMATTERS")
))]
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("NYI")?; //TODO:
		Ok(())
	}
}
#[cfg(all(
	not(any(feature = "MASM_FORMATTER", feature = "ALL_FORMATTERS")),
	not(any(feature = "NASM_FORMATTER", feature = "ALL_FORMATTERS")),
	any(feature = "INTEL_FORMATTER", feature = "ALL_FORMATTERS")
))]
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("NYI")?; //TODO:
		Ok(())
	}
}
#[cfg(all(
	not(any(feature = "MASM_FORMATTER", feature = "ALL_FORMATTERS")),
	not(any(feature = "NASM_FORMATTER", feature = "ALL_FORMATTERS")),
	not(any(feature = "INTEL_FORMATTER", feature = "ALL_FORMATTERS")),
	any(feature = "GAS_FORMATTER", feature = "ALL_FORMATTERS")
))]
impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("NYI")?; //TODO:
		Ok(())
	}
}
