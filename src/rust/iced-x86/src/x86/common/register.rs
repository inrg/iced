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

use super::icedconstants::IcedConstants;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// GENERATOR-BEGIN: Register
// This was generated by the Generator project

/// A register
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(missing_docs)]
pub enum Register {
	None,
	AL,
	CL,
	DL,
	BL,
	AH,
	CH,
	DH,
	BH,
	SPL,
	BPL,
	SIL,
	DIL,
	R8L,
	R9L,
	R10L,
	R11L,
	R12L,
	R13L,
	R14L,
	R15L,
	AX,
	CX,
	DX,
	BX,
	SP,
	BP,
	SI,
	DI,
	R8W,
	R9W,
	R10W,
	R11W,
	R12W,
	R13W,
	R14W,
	R15W,
	EAX,
	ECX,
	EDX,
	EBX,
	ESP,
	EBP,
	ESI,
	EDI,
	R8D,
	R9D,
	R10D,
	R11D,
	R12D,
	R13D,
	R14D,
	R15D,
	RAX,
	RCX,
	RDX,
	RBX,
	RSP,
	RBP,
	RSI,
	RDI,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
	EIP,
	RIP,
	ES,
	CS,
	SS,
	DS,
	FS,
	GS,
	XMM0,
	XMM1,
	XMM2,
	XMM3,
	XMM4,
	XMM5,
	XMM6,
	XMM7,
	XMM8,
	XMM9,
	XMM10,
	XMM11,
	XMM12,
	XMM13,
	XMM14,
	XMM15,
	XMM16,
	XMM17,
	XMM18,
	XMM19,
	XMM20,
	XMM21,
	XMM22,
	XMM23,
	XMM24,
	XMM25,
	XMM26,
	XMM27,
	XMM28,
	XMM29,
	XMM30,
	XMM31,
	YMM0,
	YMM1,
	YMM2,
	YMM3,
	YMM4,
	YMM5,
	YMM6,
	YMM7,
	YMM8,
	YMM9,
	YMM10,
	YMM11,
	YMM12,
	YMM13,
	YMM14,
	YMM15,
	YMM16,
	YMM17,
	YMM18,
	YMM19,
	YMM20,
	YMM21,
	YMM22,
	YMM23,
	YMM24,
	YMM25,
	YMM26,
	YMM27,
	YMM28,
	YMM29,
	YMM30,
	YMM31,
	ZMM0,
	ZMM1,
	ZMM2,
	ZMM3,
	ZMM4,
	ZMM5,
	ZMM6,
	ZMM7,
	ZMM8,
	ZMM9,
	ZMM10,
	ZMM11,
	ZMM12,
	ZMM13,
	ZMM14,
	ZMM15,
	ZMM16,
	ZMM17,
	ZMM18,
	ZMM19,
	ZMM20,
	ZMM21,
	ZMM22,
	ZMM23,
	ZMM24,
	ZMM25,
	ZMM26,
	ZMM27,
	ZMM28,
	ZMM29,
	ZMM30,
	ZMM31,
	K0,
	K1,
	K2,
	K3,
	K4,
	K5,
	K6,
	K7,
	BND0,
	BND1,
	BND2,
	BND3,
	CR0,
	CR1,
	CR2,
	CR3,
	CR4,
	CR5,
	CR6,
	CR7,
	CR8,
	CR9,
	CR10,
	CR11,
	CR12,
	CR13,
	CR14,
	CR15,
	DR0,
	DR1,
	DR2,
	DR3,
	DR4,
	DR5,
	DR6,
	DR7,
	DR8,
	DR9,
	DR10,
	DR11,
	DR12,
	DR13,
	DR14,
	DR15,
	ST0,
	ST1,
	ST2,
	ST3,
	ST4,
	ST5,
	ST6,
	ST7,
	MM0,
	MM1,
	MM2,
	MM3,
	MM4,
	MM5,
	MM6,
	MM7,
	TR0,
	TR1,
	TR2,
	TR3,
	TR4,
	TR5,
	TR6,
	TR7,
}
// GENERATOR-END: Register

impl Register {
	fn add(self, rhs: u32) -> Register {
		let result = (self as u32).wrapping_add(rhs);
		if result < IcedConstants::NUMBER_OF_REGISTERS as u32 {
			// safe: guaranteed to return a valid enum value
			unsafe { std::mem::transmute(result as u8) }
		} else {
			panic!("NYI") //TODO:
		}
	}
	fn sub(self, rhs: u32) -> Register {
		let result = (self as u32).wrapping_sub(rhs);
		if result < IcedConstants::NUMBER_OF_REGISTERS as u32 {
			// safe: guaranteed to return a valid enum value
			unsafe { std::mem::transmute(result as u8) }
		} else {
			panic!("NYI") //TODO:
		}
	}
}
// i32 + Register
impl Add<Register> for i32 {
	type Output = Register;

	fn add(self, rhs: Register) -> Self::Output {
		rhs.add(self as u32)
	}
}
// u32 + Register
impl Add<Register> for u32 {
	type Output = Register;

	fn add(self, rhs: Register) -> Self::Output {
		rhs.add(self)
	}
}
// Register + i32
impl Add<i32> for Register {
	type Output = Self;

	fn add(self, rhs: i32) -> Self::Output {
		self.add(rhs as u32)
	}
}
// Register + u32
impl Add<u32> for Register {
	type Output = Self;

	fn add(self, rhs: u32) -> Self::Output {
		self.add(rhs)
	}
}
// Register += i32
impl AddAssign<i32> for Register {
	fn add_assign(&mut self, rhs: i32) {
		*self = self.add(rhs as u32)
	}
}
// Register += u32
impl AddAssign<u32> for Register {
	fn add_assign(&mut self, rhs: u32) {
		*self = self.add(rhs)
	}
}
// Register - i32
impl Sub<i32> for Register {
	type Output = Self;

	fn sub(self, rhs: i32) -> Self::Output {
		self.sub(rhs as u32)
	}
}
// Register - u32
impl Sub<u32> for Register {
	type Output = Self;

	fn sub(self, rhs: u32) -> Self::Output {
		self.sub(rhs)
	}
}
// Register -= i32
impl SubAssign<i32> for Register {
	fn sub_assign(&mut self, rhs: i32) {
		*self = self.sub(rhs as u32)
	}
}
// Register -= u32
impl SubAssign<u32> for Register {
	fn sub_assign(&mut self, rhs: u32) {
		*self = self.sub(rhs)
	}
}
