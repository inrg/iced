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

#[cfg(feature = "INSTR_INFO")]
pub use info::*;

#[cfg(feature = "INSTR_INFO")]
mod info {
	use super::super::icedconstants::IcedConstants;
	use super::MemorySize;
	use std::mem;

	#[rustfmt::skip]
	pub(crate) static MEMORY_SIZE_INFOS: &[MemorySizeInfo; IcedConstants::NUMBER_OF_MEMORY_SIZES as usize] = &[
		MemorySizeInfo { size: 0, element_size: 0, memory_size: MemorySize::Unknown as u8, element_type: MemorySize::Unknown as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 1, element_size: 1, memory_size: MemorySize::UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 2, memory_size: MemorySize::UInt16 as u8, element_type: MemorySize::UInt16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 16, memory_size: MemorySize::UInt128 as u8, element_type: MemorySize::UInt128 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 32, memory_size: MemorySize::UInt256 as u8, element_type: MemorySize::UInt256 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 64, memory_size: MemorySize::UInt512 as u8, element_type: MemorySize::UInt512 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 1, element_size: 1, memory_size: MemorySize::Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 2, memory_size: MemorySize::Int16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 16, memory_size: MemorySize::Int128 as u8, element_type: MemorySize::Int128 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 32, memory_size: MemorySize::Int256 as u8, element_type: MemorySize::Int256 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 64, memory_size: MemorySize::Int512 as u8, element_type: MemorySize::Int512 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::SegPtr16 as u8, element_type: MemorySize::SegPtr16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 6, element_size: 6, memory_size: MemorySize::SegPtr32 as u8, element_type: MemorySize::SegPtr32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 10, element_size: 10, memory_size: MemorySize::SegPtr64 as u8, element_type: MemorySize::SegPtr64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 2, memory_size: MemorySize::WordOffset as u8, element_type: MemorySize::WordOffset as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::DwordOffset as u8, element_type: MemorySize::DwordOffset as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::QwordOffset as u8, element_type: MemorySize::QwordOffset as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Bound16_WordWord as u8, element_type: MemorySize::Bound16_WordWord as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Bound32_DwordDword as u8, element_type: MemorySize::Bound32_DwordDword as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Bnd32 as u8, element_type: MemorySize::Bnd32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 16, memory_size: MemorySize::Bnd64 as u8, element_type: MemorySize::Bnd64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 6, element_size: 6, memory_size: MemorySize::Fword6 as u8, element_type: MemorySize::Fword6 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 10, element_size: 10, memory_size: MemorySize::Fword10 as u8, element_type: MemorySize::Fword10 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 2, memory_size: MemorySize::Float16 as u8, element_type: MemorySize::Float16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 10, element_size: 10, memory_size: MemorySize::Float80 as u8, element_type: MemorySize::Float80 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 16, memory_size: MemorySize::Float128 as u8, element_type: MemorySize::Float128 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 2, memory_size: MemorySize::BFloat16 as u8, element_type: MemorySize::BFloat16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 14, element_size: 14, memory_size: MemorySize::FpuEnv14 as u8, element_type: MemorySize::FpuEnv14 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 28, element_size: 28, memory_size: MemorySize::FpuEnv28 as u8, element_type: MemorySize::FpuEnv28 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 94, element_size: 94, memory_size: MemorySize::FpuState94 as u8, element_type: MemorySize::FpuState94 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 108, element_size: 108, memory_size: MemorySize::FpuState108 as u8, element_type: MemorySize::FpuState108 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 512, element_size: 512, memory_size: MemorySize::Fxsave_512Byte as u8, element_type: MemorySize::Fxsave_512Byte as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 512, element_size: 512, memory_size: MemorySize::Fxsave64_512Byte as u8, element_type: MemorySize::Fxsave64_512Byte as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 0, element_size: 0, memory_size: MemorySize::Xsave as u8, element_type: MemorySize::Xsave as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 0, element_size: 0, memory_size: MemorySize::Xsave64 as u8, element_type: MemorySize::Xsave64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 10, element_size: 10, memory_size: MemorySize::Bcd as u8, element_type: MemorySize::Bcd as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 1, memory_size: MemorySize::Packed16_UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 2, element_size: 1, memory_size: MemorySize::Packed16_Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 1, memory_size: MemorySize::Packed32_UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 1, memory_size: MemorySize::Packed32_Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Packed32_UInt16 as u8, element_type: MemorySize::UInt16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Packed32_Int16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Packed32_BFloat16 as u8, element_type: MemorySize::BFloat16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 1, memory_size: MemorySize::Packed64_UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 1, memory_size: MemorySize::Packed64_Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 2, memory_size: MemorySize::Packed64_UInt16 as u8, element_type: MemorySize::UInt16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 2, memory_size: MemorySize::Packed64_Int16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Packed64_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Packed64_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 2, memory_size: MemorySize::Packed64_Float16 as u8, element_type: MemorySize::Float16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Packed64_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 1, memory_size: MemorySize::Packed128_UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 1, memory_size: MemorySize::Packed128_Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 2, memory_size: MemorySize::Packed128_UInt16 as u8, element_type: MemorySize::UInt16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 2, memory_size: MemorySize::Packed128_Int16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 4, memory_size: MemorySize::Packed128_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 4, memory_size: MemorySize::Packed128_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 8, memory_size: MemorySize::Packed128_UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 8, memory_size: MemorySize::Packed128_UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 8, memory_size: MemorySize::Packed128_Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 2, memory_size: MemorySize::Packed128_Float16 as u8, element_type: MemorySize::Float16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 4, memory_size: MemorySize::Packed128_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 8, memory_size: MemorySize::Packed128_Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 16, element_size: 4, memory_size: MemorySize::Packed128_2xBFloat16 as u8, element_type: MemorySize::Packed32_BFloat16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 1, memory_size: MemorySize::Packed256_UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 1, memory_size: MemorySize::Packed256_Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 2, memory_size: MemorySize::Packed256_UInt16 as u8, element_type: MemorySize::UInt16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 2, memory_size: MemorySize::Packed256_Int16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 4, memory_size: MemorySize::Packed256_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 4, memory_size: MemorySize::Packed256_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 8, memory_size: MemorySize::Packed256_UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 8, memory_size: MemorySize::Packed256_UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 8, memory_size: MemorySize::Packed256_Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 16, memory_size: MemorySize::Packed256_UInt128 as u8, element_type: MemorySize::UInt128 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 16, memory_size: MemorySize::Packed256_Int128 as u8, element_type: MemorySize::Int128 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 2, memory_size: MemorySize::Packed256_Float16 as u8, element_type: MemorySize::Float16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 4, memory_size: MemorySize::Packed256_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 8, memory_size: MemorySize::Packed256_Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 16, memory_size: MemorySize::Packed256_Float128 as u8, element_type: MemorySize::Float128 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 32, element_size: 4, memory_size: MemorySize::Packed256_2xBFloat16 as u8, element_type: MemorySize::Packed32_BFloat16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 1, memory_size: MemorySize::Packed512_UInt8 as u8, element_type: MemorySize::UInt8 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 1, memory_size: MemorySize::Packed512_Int8 as u8, element_type: MemorySize::Int8 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 2, memory_size: MemorySize::Packed512_UInt16 as u8, element_type: MemorySize::UInt16 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 2, memory_size: MemorySize::Packed512_Int16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 4, memory_size: MemorySize::Packed512_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 4, memory_size: MemorySize::Packed512_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 8, memory_size: MemorySize::Packed512_UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 8, memory_size: MemorySize::Packed512_UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 8, memory_size: MemorySize::Packed512_Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 16, memory_size: MemorySize::Packed512_UInt128 as u8, element_type: MemorySize::UInt128 as u8, is_signed: false, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 4, memory_size: MemorySize::Packed512_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 8, memory_size: MemorySize::Packed512_Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 64, element_size: 4, memory_size: MemorySize::Packed512_2xBFloat16 as u8, element_type: MemorySize::Packed32_BFloat16 as u8, is_signed: true, is_broadcast: false },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast64_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast64_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast64_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast128_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast128_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast128_UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast128_UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast128_Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast128_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast128_Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast256_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast256_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast256_UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast256_UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast256_Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast256_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast256_Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast512_UInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast512_Int32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast512_UInt52 as u8, element_type: MemorySize::UInt52 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast512_UInt64 as u8, element_type: MemorySize::UInt64 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast512_Int64 as u8, element_type: MemorySize::Int64 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 4, memory_size: MemorySize::Broadcast512_Float32 as u8, element_type: MemorySize::Float32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 8, memory_size: MemorySize::Broadcast512_Float64 as u8, element_type: MemorySize::Float64 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Broadcast128_2xInt16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Broadcast256_2xInt16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Broadcast512_2xInt16 as u8, element_type: MemorySize::Int16 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Broadcast128_2xUInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Broadcast256_2xUInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Broadcast512_2xUInt32 as u8, element_type: MemorySize::UInt32 as u8, is_signed: false, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Broadcast128_2xInt32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Broadcast256_2xInt32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 8, element_size: 4, memory_size: MemorySize::Broadcast512_2xInt32 as u8, element_type: MemorySize::Int32 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Broadcast128_2xBFloat16 as u8, element_type: MemorySize::BFloat16 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Broadcast256_2xBFloat16 as u8, element_type: MemorySize::BFloat16 as u8, is_signed: true, is_broadcast: true },
		MemorySizeInfo { size: 4, element_size: 2, memory_size: MemorySize::Broadcast512_2xBFloat16 as u8, element_type: MemorySize::BFloat16 as u8, is_signed: true, is_broadcast: true },
	];

	/// `MemorySize` information
	#[derive(Copy, Clone)]
	pub struct MemorySizeInfo {
		size: u16,
		element_size: u16,
		memory_size: u8,
		element_type: u8,
		// Use flags if more booleans are needed
		is_signed: bool,
		is_broadcast: bool,
	}

	#[allow(clippy::trivially_copy_pass_by_ref)]
	impl MemorySizeInfo {
		/// Gets the `MemorySize` value
		#[inline]
		pub fn memory_size(&self) -> MemorySize {
			// safe: memory_size is always a valid MemorySize value
			unsafe { mem::transmute(self.memory_size) }
		}

		/// Gets the size in bytes of the memory location or 0 if it's not accessed or unknown
		#[inline]
		pub fn size(&self) -> i32 {
			self.size as i32
		}

		/// Gets the size in bytes of the packed element. If it's not a packed data type, it's equal to `size()`.
		#[inline]
		pub fn element_size(&self) -> i32 {
			self.element_size as i32
		}

		/// Gets the element type if it's packed data or the type itself if it's not packed data
		#[inline]
		pub fn element_type(&self) -> MemorySize {
			// safe: element_type is always a valid MemorySize value
			unsafe { mem::transmute(self.element_type) }
		}

		/// Gets the element type if it's packed data or the type itself if it's not packed data
		#[inline]
		pub fn element_type_info(&self) -> &'static MemorySizeInfo {
			self.element_type().info()
		}

		/// `true` if it's signed data (signed integer or a floating point value)
		#[inline]
		pub fn is_signed(&self) -> bool {
			self.is_signed
		}

		/// `true` if it's a broadcast memory type
		#[inline]
		pub fn is_broadcast(&self) -> bool {
			self.is_broadcast
		}

		/// `true` if this is a packed data type, eg. `MemorySize::Packed128_Float32`. See also `element_count()`
		#[inline]
		pub fn is_packed(&self) -> bool {
			self.element_size < self.size
		}

		/// Gets the number of elements in the packed data type or `1` if it's not packed data (`is_packed()`)
		#[inline]
		pub fn element_count(&self) -> i32 {
			// element_size can be 0 so we don't divide by it if es == s
			if self.element_size == self.size {
				1
			} else {
				self.size as i32 / self.element_size as i32
			}
		}
	}
}

// GENERATOR-BEGIN: MemorySize
// This was generated by the Generator project

/// Size of a memory reference
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum MemorySize {
	/// Unknown size or the instruction doesn't reference any memory (eg. `lea`)
	Unknown,
	/// Memory location contains a `u8`
	UInt8,
	/// Memory location contains a `u16`
	UInt16,
	/// Memory location contains a `u32`
	UInt32,
	/// Memory location contains a `u52`
	UInt52,
	/// Memory location contains a `u64`
	UInt64,
	/// Memory location contains a `u128`
	UInt128,
	/// Memory location contains a `u256`
	UInt256,
	/// Memory location contains a `u512`
	UInt512,
	/// Memory location contains a `i8`
	Int8,
	/// Memory location contains a `i16`
	Int16,
	/// Memory location contains a `i32`
	Int32,
	/// Memory location contains a `i64`
	Int64,
	/// Memory location contains a `i128`
	Int128,
	/// Memory location contains a `i256`
	Int256,
	/// Memory location contains a `i512`
	Int512,
	/// Memory location contains a seg:ptr pair, `u16` (offset) + `u16` (segment/selector)
	SegPtr16,
	/// Memory location contains a seg:ptr pair, `u32` (offset) + `u16` (segment/selector)
	SegPtr32,
	/// Memory location contains a seg:ptr pair, `u64` (offset) + `u16` (segment/selector)
	SegPtr64,
	/// Memory location contains a 16-bit offset (`jmp/call word ptr [mem]`)
	WordOffset,
	/// Memory location contains a 32-bit offset (`jmp/call dword ptr [mem]`)
	DwordOffset,
	/// Memory location contains a 64-bit offset (`jmp/call qword ptr [mem]`)
	QwordOffset,
	/// Memory location contains two `u16`s (16-bit `bound`)
	Bound16_WordWord,
	/// Memory location contains two `u32`s (32-bit `bound`)
	Bound32_DwordDword,
	/// 32-bit `bndmov`, 2 x `u32`
	Bnd32,
	/// 64-bit `bndmov`, 2 x `u64`
	Bnd64,
	/// Memory location contains a 16-bit limit and a 32-bit address (eg. `lgdtw`, `lgdtd`)
	Fword6,
	/// Memory location contains a 16-bit limit and a 64-bit address (eg. `lgdtq`)
	Fword10,
	/// Memory location contains a `f16`
	Float16,
	/// Memory location contains a `f32`
	Float32,
	/// Memory location contains a `f64`
	Float64,
	/// Memory location contains a `f80`
	Float80,
	/// Memory location contains a `f128`
	Float128,
	/// Memory location contains a `bfloat16`
	BFloat16,
	/// Memory location contains a 14-byte FPU environment (16-bit `fldenv`/`fstenv`)
	FpuEnv14,
	/// Memory location contains a 28-byte FPU environment (32/64-bit `fldenv`/`fstenv`)
	FpuEnv28,
	/// Memory location contains a 94-byte FPU environment (16-bit `fsave`/`frstor`)
	FpuState94,
	/// Memory location contains a 108-byte FPU environment (32/64-bit `fsave`/`frstor`)
	FpuState108,
	/// Memory location contains 512-bytes of `fxsave`/`fxrstor` data
	Fxsave_512Byte,
	/// Memory location contains 512-bytes of `fxsave64`/`fxrstor64` data
	Fxsave64_512Byte,
	/// 32-bit `XSAVE` area
	Xsave,
	/// 64-bit `XSAVE` area
	Xsave64,
	/// Memory location contains a 10-byte `bcd` value (`fbld`/`fbstp`)
	Bcd,
	/// 16 bit location: 2 x `u8`
	Packed16_UInt8,
	/// 16 bit location: 2 x `i8`
	Packed16_Int8,
	/// 32 bit location: 4 x `u8`
	Packed32_UInt8,
	/// 32 bit location: 4 x `i8`
	Packed32_Int8,
	/// 32 bit location: 2 x `u16`
	Packed32_UInt16,
	/// 32 bit location: 2 x `i16`
	Packed32_Int16,
	/// 32 bit location: 2 x `bfloat16`
	Packed32_BFloat16,
	/// 64-bit location: 8 x `u8`
	Packed64_UInt8,
	/// 64-bit location: 8 x `i8`
	Packed64_Int8,
	/// 64-bit location: 4 x `u16`
	Packed64_UInt16,
	/// 64-bit location: 4 x `i16`
	Packed64_Int16,
	/// 64-bit location: 2 x `u32`
	Packed64_UInt32,
	/// 64-bit location: 2 x `i32`
	Packed64_Int32,
	/// 64-bit location: 4 x `f16`
	Packed64_Float16,
	/// 64-bit location: 2 x `f32`
	Packed64_Float32,
	/// 128 bit location: 16 x `u8`
	Packed128_UInt8,
	/// 128 bit location: 16 x `i8`
	Packed128_Int8,
	/// 128 bit location: 8 x `u16`
	Packed128_UInt16,
	/// 128 bit location: 8 x `i16`
	Packed128_Int16,
	/// 128 bit location: 4 x `u32`
	Packed128_UInt32,
	/// 128 bit location: 4 x `i32`
	Packed128_Int32,
	/// 128 bit location: 2 x `u52`
	Packed128_UInt52,
	/// 128 bit location: 2 x `u64`
	Packed128_UInt64,
	/// 128 bit location: 2 x `i64`
	Packed128_Int64,
	/// 128 bit location: 8 x `f16`
	Packed128_Float16,
	/// 128 bit location: 4 x `f32`
	Packed128_Float32,
	/// 128 bit location: 2 x `f64`
	Packed128_Float64,
	/// 128 bit location: 4 x (2 x `bfloat16`)
	Packed128_2xBFloat16,
	/// 256 bit location: 32 x `u8`
	Packed256_UInt8,
	/// 256 bit location: 32 x `i8`
	Packed256_Int8,
	/// 256 bit location: 16 x `u16`
	Packed256_UInt16,
	/// 256 bit location: 16 x `i16`
	Packed256_Int16,
	/// 256 bit location: 8 x `u32`
	Packed256_UInt32,
	/// 256 bit location: 8 x `i32`
	Packed256_Int32,
	/// 256 bit location: 4 x `u52`
	Packed256_UInt52,
	/// 256 bit location: 4 x `u64`
	Packed256_UInt64,
	/// 256 bit location: 4 x `i64`
	Packed256_Int64,
	/// 256 bit location: 2 x `u128`
	Packed256_UInt128,
	/// 256 bit location: 2 x `i128`
	Packed256_Int128,
	/// 256 bit location: 16 x `f16`
	Packed256_Float16,
	/// 256 bit location: 8 x `f32`
	Packed256_Float32,
	/// 256 bit location: 4 x `f64`
	Packed256_Float64,
	/// 256 bit location: 2 x `f128`
	Packed256_Float128,
	/// 256 bit location: 8 x (2 x `bfloat16`)
	Packed256_2xBFloat16,
	/// 512 bit location: 64 x `u8`
	Packed512_UInt8,
	/// 512 bit location: 64 x `i8`
	Packed512_Int8,
	/// 512 bit location: 32 x `u16`
	Packed512_UInt16,
	/// 512 bit location: 32 x `i16`
	Packed512_Int16,
	/// 512 bit location: 16 x `u32`
	Packed512_UInt32,
	/// 512 bit location: 16 x `i32`
	Packed512_Int32,
	/// 512 bit location: 8 x `u52`
	Packed512_UInt52,
	/// 512 bit location: 8 x `u64`
	Packed512_UInt64,
	/// 512 bit location: 8 x `i64`
	Packed512_Int64,
	/// 256 bit location: 4 x `u128`
	Packed512_UInt128,
	/// 512 bit location: 16 x `f32`
	Packed512_Float32,
	/// 512 bit location: 8 x `f64`
	Packed512_Float64,
	/// 512 bit location: 16 x (2 x `bfloat16`)
	Packed512_2xBFloat16,
	/// Broadcast `u32` to 64 bits
	Broadcast64_UInt32,
	/// Broadcast `i32` to 64 bits
	Broadcast64_Int32,
	/// Broadcast `f32` to 64 bits
	Broadcast64_Float32,
	/// Broadcast `u32` to 128 bits
	Broadcast128_UInt32,
	/// Broadcast `i32` to 128 bits
	Broadcast128_Int32,
	/// Broadcast `u52` to 128 bits
	Broadcast128_UInt52,
	/// Broadcast `u64` to 128 bits
	Broadcast128_UInt64,
	/// Broadcast `i64` to 128 bits
	Broadcast128_Int64,
	/// Broadcast `f32` to 128 bits
	Broadcast128_Float32,
	/// Broadcast `f64` to 128 bits
	Broadcast128_Float64,
	/// Broadcast `u32` to 256 bits
	Broadcast256_UInt32,
	/// Broadcast `i32` to 256 bits
	Broadcast256_Int32,
	/// Broadcast `u52` to 256 bits
	Broadcast256_UInt52,
	/// Broadcast `u64` to 256 bits
	Broadcast256_UInt64,
	/// Broadcast `i64` to 256 bits
	Broadcast256_Int64,
	/// Broadcast `f32` to 256 bits
	Broadcast256_Float32,
	/// Broadcast `f64` to 256 bits
	Broadcast256_Float64,
	/// Broadcast `u32` to 512 bits
	Broadcast512_UInt32,
	/// Broadcast `i32` to 512 bits
	Broadcast512_Int32,
	/// Broadcast `u52` to 512 bits
	Broadcast512_UInt52,
	/// Broadcast `u64` to 512 bits
	Broadcast512_UInt64,
	/// Broadcast `i64` to 512 bits
	Broadcast512_Int64,
	/// Broadcast `f32` to 512 bits
	Broadcast512_Float32,
	/// Broadcast `f64` to 512 bits
	Broadcast512_Float64,
	/// Broadcast 2 x `i16` to 128 bits
	Broadcast128_2xInt16,
	/// Broadcast 2 x `i16` to 256 bits
	Broadcast256_2xInt16,
	/// Broadcast 2 x `i16` to 512 bits
	Broadcast512_2xInt16,
	/// Broadcast 2 x `u32` to 128 bits
	Broadcast128_2xUInt32,
	/// Broadcast 2 x `u32` to 256 bits
	Broadcast256_2xUInt32,
	/// Broadcast 2 x `u32` to 512 bits
	Broadcast512_2xUInt32,
	/// Broadcast 2 x `i32` to 128 bits
	Broadcast128_2xInt32,
	/// Broadcast 2 x `i32` to 256 bits
	Broadcast256_2xInt32,
	/// Broadcast 2 x `i32` to 512 bits
	Broadcast512_2xInt32,
	/// Broadcast 2 x `bfloat16` to 128 bits
	Broadcast128_2xBFloat16,
	/// Broadcast 2 x `bfloat16` to 256 bits
	Broadcast256_2xBFloat16,
	/// Broadcast 2 x `bfloat16` to 512 bits
	Broadcast512_2xBFloat16,
}
// GENERATOR-END: MemorySize

#[cfg(feature = "INSTR_INFO")]
impl MemorySize {
	/// Gets the memory size info
	#[inline]
	pub fn info(self) -> &'static MemorySizeInfo {
		&MEMORY_SIZE_INFOS[self as usize]
	}

	/// Gets the size in bytes of the memory location or 0 if it's not accessed by the instruction or unknown or variable sized
	#[inline]
	pub fn size(self) -> i32 {
		self.info().size()
	}

	/// Gets the size in bytes of the packed element. If it's not a packed data type, it's equal to `size()`.
	#[inline]
	pub fn element_size(self) -> i32 {
		self.info().element_size()
	}

	/// Gets the element type if it's packed data or `self` if it's not packed data
	#[inline]
	pub fn element_type(self) -> MemorySize {
		self.info().element_type()
	}

	/// true if it's signed data (signed integer or a floating point value)
	#[inline]
	pub fn is_signed(self) -> bool {
		self.info().is_signed()
	}

	/// true if this is a packed data type, eg. `MemorySize::Packed128_Float32`
	#[inline]
	pub fn is_packed(self) -> bool {
		self.info().is_packed()
	}

	/// Gets the number of elements in the packed data type or `1` if it's not packed data (`is_packed()`)
	#[inline]
	pub fn element_count(self) -> i32 {
		self.info().element_count()
	}
}

#[cfg(any(feature = "INSTR_INFO", feature = "ENCODER"))]
impl MemorySize {
	/// Checks if it is a broadcast memory type
	#[inline]
	pub fn is_broadcast(self) -> bool {
		self >= IcedConstants::FIRST_BROADCAST_MEMORY_SIZE
	}
}
