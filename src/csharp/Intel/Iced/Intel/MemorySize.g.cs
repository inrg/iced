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

// This file was generated by the Generator project

#nullable enable

namespace Iced.Intel {
	/// <summary>Size of a memory reference</summary>
	public enum MemorySize {
		/// <summary>Unknown size or the instruction doesn&apos;t reference any memory (eg. <c>lea</c>)</summary>
		Unknown,
		/// <summary>Memory location contains a <see cref="byte"/></summary>
		UInt8,
		/// <summary>Memory location contains a <see cref="ushort"/></summary>
		UInt16,
		/// <summary>Memory location contains a <see cref="uint"/></summary>
		UInt32,
		/// <summary>Memory location contains a <c>uint52</c></summary>
		UInt52,
		/// <summary>Memory location contains a <see cref="ulong"/></summary>
		UInt64,
		/// <summary>Memory location contains a <c>uint128</c></summary>
		UInt128,
		/// <summary>Memory location contains a <c>uint256</c></summary>
		UInt256,
		/// <summary>Memory location contains a <c>uint512</c></summary>
		UInt512,
		/// <summary>Memory location contains a <see cref="sbyte"/></summary>
		Int8,
		/// <summary>Memory location contains a <see cref="short"/></summary>
		Int16,
		/// <summary>Memory location contains a <see cref="int"/></summary>
		Int32,
		/// <summary>Memory location contains a <see cref="long"/></summary>
		Int64,
		/// <summary>Memory location contains a <c>int128</c></summary>
		Int128,
		/// <summary>Memory location contains a <c>int256</c></summary>
		Int256,
		/// <summary>Memory location contains a <c>int512</c></summary>
		Int512,
		/// <summary>Memory location contains a seg:ptr pair, <see cref="ushort"/> (offset) + <see cref="ushort"/> (segment/selector)</summary>
		SegPtr16,
		/// <summary>Memory location contains a seg:ptr pair, <see cref="uint"/> (offset) + <see cref="ushort"/> (segment/selector)</summary>
		SegPtr32,
		/// <summary>Memory location contains a seg:ptr pair, <see cref="ulong"/> (offset) + <see cref="ushort"/> (segment/selector)</summary>
		SegPtr64,
		/// <summary>Memory location contains a 16-bit offset (<c>jmp/call word ptr [mem]</c>)</summary>
		WordOffset,
		/// <summary>Memory location contains a 32-bit offset (<c>jmp/call dword ptr [mem]</c>)</summary>
		DwordOffset,
		/// <summary>Memory location contains a 64-bit offset (<c>jmp/call qword ptr [mem]</c>)</summary>
		QwordOffset,
		/// <summary>Memory location contains two <see cref="ushort"/>s (16-bit <c>bound</c>)</summary>
		Bound16_WordWord,
		/// <summary>Memory location contains two <see cref="uint"/>s (32-bit <c>bound</c>)</summary>
		Bound32_DwordDword,
		/// <summary>32-bit <c>bndmov</c>, 2 x <see cref="uint"/></summary>
		Bnd32,
		/// <summary>64-bit <c>bndmov</c>, 2 x <see cref="ulong"/></summary>
		Bnd64,
		/// <summary>Memory location contains a 16-bit limit and a 32-bit address (eg. <c>lgdtw</c>, <c>lgdtd</c>)</summary>
		Fword6,
		/// <summary>Memory location contains a 16-bit limit and a 64-bit address (eg. <c>lgdtq</c>)</summary>
		Fword10,
		/// <summary>Memory location contains a <c>float16</c></summary>
		Float16,
		/// <summary>Memory location contains a <see cref="float"/></summary>
		Float32,
		/// <summary>Memory location contains a <see cref="double"/></summary>
		Float64,
		/// <summary>Memory location contains a <c>float80</c></summary>
		Float80,
		/// <summary>Memory location contains a <c>float128</c></summary>
		Float128,
		/// <summary>Memory location contains a <c>bfloat16</c></summary>
		BFloat16,
		/// <summary>Memory location contains a 14-byte FPU environment (16-bit <c>fldenv</c>/<c>fstenv</c>)</summary>
		FpuEnv14,
		/// <summary>Memory location contains a 28-byte FPU environment (32/64-bit <c>fldenv</c>/<c>fstenv</c>)</summary>
		FpuEnv28,
		/// <summary>Memory location contains a 94-byte FPU environment (16-bit <c>fsave</c>/<c>frstor</c>)</summary>
		FpuState94,
		/// <summary>Memory location contains a 108-byte FPU environment (32/64-bit <c>fsave</c>/<c>frstor</c>)</summary>
		FpuState108,
		/// <summary>Memory location contains 512-bytes of <c>fxsave</c>/<c>fxrstor</c> data</summary>
		Fxsave_512Byte,
		/// <summary>Memory location contains 512-bytes of <c>fxsave64</c>/<c>fxrstor64</c> data</summary>
		Fxsave64_512Byte,
		/// <summary>32-bit <c>XSAVE</c> area</summary>
		Xsave,
		/// <summary>64-bit <c>XSAVE</c> area</summary>
		Xsave64,
		/// <summary>Memory location contains a 10-byte <c>bcd</c> value (<c>fbld</c>/<c>fbstp</c>)</summary>
		Bcd,
		/// <summary>16 bit location: 2 x <see cref="byte"/></summary>
		Packed16_UInt8,
		/// <summary>16 bit location: 2 x <see cref="sbyte"/></summary>
		Packed16_Int8,
		/// <summary>32 bit location: 4 x <see cref="byte"/></summary>
		Packed32_UInt8,
		/// <summary>32 bit location: 4 x <see cref="sbyte"/></summary>
		Packed32_Int8,
		/// <summary>32 bit location: 2 x <see cref="ushort"/></summary>
		Packed32_UInt16,
		/// <summary>32 bit location: 2 x <see cref="short"/></summary>
		Packed32_Int16,
		/// <summary>32 bit location: 2 x <c>bfloat16</c></summary>
		Packed32_BFloat16,
		/// <summary>64-bit location: 8 x <see cref="byte"/></summary>
		Packed64_UInt8,
		/// <summary>64-bit location: 8 x <see cref="sbyte"/></summary>
		Packed64_Int8,
		/// <summary>64-bit location: 4 x <see cref="ushort"/></summary>
		Packed64_UInt16,
		/// <summary>64-bit location: 4 x <see cref="short"/></summary>
		Packed64_Int16,
		/// <summary>64-bit location: 2 x <see cref="uint"/></summary>
		Packed64_UInt32,
		/// <summary>64-bit location: 2 x <see cref="int"/></summary>
		Packed64_Int32,
		/// <summary>64-bit location: 4 x <c>float16</c></summary>
		Packed64_Float16,
		/// <summary>64-bit location: 2 x <see cref="float"/></summary>
		Packed64_Float32,
		/// <summary>128 bit location: 16 x <see cref="byte"/></summary>
		Packed128_UInt8,
		/// <summary>128 bit location: 16 x <see cref="sbyte"/></summary>
		Packed128_Int8,
		/// <summary>128 bit location: 8 x <see cref="ushort"/></summary>
		Packed128_UInt16,
		/// <summary>128 bit location: 8 x <see cref="short"/></summary>
		Packed128_Int16,
		/// <summary>128 bit location: 4 x <see cref="uint"/></summary>
		Packed128_UInt32,
		/// <summary>128 bit location: 4 x <see cref="int"/></summary>
		Packed128_Int32,
		/// <summary>128 bit location: 2 x <c>uint52</c></summary>
		Packed128_UInt52,
		/// <summary>128 bit location: 2 x <see cref="ulong"/></summary>
		Packed128_UInt64,
		/// <summary>128 bit location: 2 x <see cref="long"/></summary>
		Packed128_Int64,
		/// <summary>128 bit location: 8 x <c>float16</c></summary>
		Packed128_Float16,
		/// <summary>128 bit location: 4 x <see cref="float"/></summary>
		Packed128_Float32,
		/// <summary>128 bit location: 2 x <see cref="double"/></summary>
		Packed128_Float64,
		/// <summary>128 bit location: 4 x (2 x <c>bfloat16</c>)</summary>
		Packed128_2xBFloat16,
		/// <summary>256 bit location: 32 x <see cref="byte"/></summary>
		Packed256_UInt8,
		/// <summary>256 bit location: 32 x <see cref="sbyte"/></summary>
		Packed256_Int8,
		/// <summary>256 bit location: 16 x <see cref="ushort"/></summary>
		Packed256_UInt16,
		/// <summary>256 bit location: 16 x <see cref="short"/></summary>
		Packed256_Int16,
		/// <summary>256 bit location: 8 x <see cref="uint"/></summary>
		Packed256_UInt32,
		/// <summary>256 bit location: 8 x <see cref="int"/></summary>
		Packed256_Int32,
		/// <summary>256 bit location: 4 x <c>uint52</c></summary>
		Packed256_UInt52,
		/// <summary>256 bit location: 4 x <see cref="ulong"/></summary>
		Packed256_UInt64,
		/// <summary>256 bit location: 4 x <see cref="long"/></summary>
		Packed256_Int64,
		/// <summary>256 bit location: 2 x <c>uint128</c></summary>
		Packed256_UInt128,
		/// <summary>256 bit location: 2 x <c>int128</c></summary>
		Packed256_Int128,
		/// <summary>256 bit location: 16 x <c>float16</c></summary>
		Packed256_Float16,
		/// <summary>256 bit location: 8 x <see cref="float"/></summary>
		Packed256_Float32,
		/// <summary>256 bit location: 4 x <see cref="double"/></summary>
		Packed256_Float64,
		/// <summary>256 bit location: 2 x <c>float128</c></summary>
		Packed256_Float128,
		/// <summary>256 bit location: 8 x (2 x <c>bfloat16</c>)</summary>
		Packed256_2xBFloat16,
		/// <summary>512 bit location: 64 x <see cref="byte"/></summary>
		Packed512_UInt8,
		/// <summary>512 bit location: 64 x <see cref="sbyte"/></summary>
		Packed512_Int8,
		/// <summary>512 bit location: 32 x <see cref="ushort"/></summary>
		Packed512_UInt16,
		/// <summary>512 bit location: 32 x <see cref="short"/></summary>
		Packed512_Int16,
		/// <summary>512 bit location: 16 x <see cref="uint"/></summary>
		Packed512_UInt32,
		/// <summary>512 bit location: 16 x <see cref="int"/></summary>
		Packed512_Int32,
		/// <summary>512 bit location: 8 x <c>uint52</c></summary>
		Packed512_UInt52,
		/// <summary>512 bit location: 8 x <see cref="ulong"/></summary>
		Packed512_UInt64,
		/// <summary>512 bit location: 8 x <see cref="long"/></summary>
		Packed512_Int64,
		/// <summary>256 bit location: 4 x <c>uint128</c></summary>
		Packed512_UInt128,
		/// <summary>512 bit location: 16 x <see cref="float"/></summary>
		Packed512_Float32,
		/// <summary>512 bit location: 8 x <see cref="double"/></summary>
		Packed512_Float64,
		/// <summary>512 bit location: 16 x (2 x <c>bfloat16</c>)</summary>
		Packed512_2xBFloat16,
		/// <summary>Broadcast <see cref="uint"/> to 64 bits</summary>
		Broadcast64_UInt32,
		/// <summary>Broadcast <see cref="int"/> to 64 bits</summary>
		Broadcast64_Int32,
		/// <summary>Broadcast <see cref="float"/> to 64 bits</summary>
		Broadcast64_Float32,
		/// <summary>Broadcast <see cref="uint"/> to 128 bits</summary>
		Broadcast128_UInt32,
		/// <summary>Broadcast <see cref="int"/> to 128 bits</summary>
		Broadcast128_Int32,
		/// <summary>Broadcast <c>uint52</c> to 128 bits</summary>
		Broadcast128_UInt52,
		/// <summary>Broadcast <see cref="ulong"/> to 128 bits</summary>
		Broadcast128_UInt64,
		/// <summary>Broadcast <see cref="long"/> to 128 bits</summary>
		Broadcast128_Int64,
		/// <summary>Broadcast <see cref="float"/> to 128 bits</summary>
		Broadcast128_Float32,
		/// <summary>Broadcast <see cref="double"/> to 128 bits</summary>
		Broadcast128_Float64,
		/// <summary>Broadcast <see cref="uint"/> to 256 bits</summary>
		Broadcast256_UInt32,
		/// <summary>Broadcast <see cref="int"/> to 256 bits</summary>
		Broadcast256_Int32,
		/// <summary>Broadcast <c>uint52</c> to 256 bits</summary>
		Broadcast256_UInt52,
		/// <summary>Broadcast <see cref="ulong"/> to 256 bits</summary>
		Broadcast256_UInt64,
		/// <summary>Broadcast <see cref="long"/> to 256 bits</summary>
		Broadcast256_Int64,
		/// <summary>Broadcast <see cref="float"/> to 256 bits</summary>
		Broadcast256_Float32,
		/// <summary>Broadcast <see cref="double"/> to 256 bits</summary>
		Broadcast256_Float64,
		/// <summary>Broadcast <see cref="uint"/> to 512 bits</summary>
		Broadcast512_UInt32,
		/// <summary>Broadcast <see cref="int"/> to 512 bits</summary>
		Broadcast512_Int32,
		/// <summary>Broadcast <c>uint52</c> to 512 bits</summary>
		Broadcast512_UInt52,
		/// <summary>Broadcast <see cref="ulong"/> to 512 bits</summary>
		Broadcast512_UInt64,
		/// <summary>Broadcast <see cref="long"/> to 512 bits</summary>
		Broadcast512_Int64,
		/// <summary>Broadcast <see cref="float"/> to 512 bits</summary>
		Broadcast512_Float32,
		/// <summary>Broadcast <see cref="double"/> to 512 bits</summary>
		Broadcast512_Float64,
		/// <summary>Broadcast 2 x <see cref="short"/> to 128 bits</summary>
		Broadcast128_2xInt16,
		/// <summary>Broadcast 2 x <see cref="short"/> to 256 bits</summary>
		Broadcast256_2xInt16,
		/// <summary>Broadcast 2 x <see cref="short"/> to 512 bits</summary>
		Broadcast512_2xInt16,
		/// <summary>Broadcast 2 x <see cref="uint"/> to 128 bits</summary>
		Broadcast128_2xUInt32,
		/// <summary>Broadcast 2 x <see cref="uint"/> to 256 bits</summary>
		Broadcast256_2xUInt32,
		/// <summary>Broadcast 2 x <see cref="uint"/> to 512 bits</summary>
		Broadcast512_2xUInt32,
		/// <summary>Broadcast 2 x <see cref="int"/> to 128 bits</summary>
		Broadcast128_2xInt32,
		/// <summary>Broadcast 2 x <see cref="int"/> to 256 bits</summary>
		Broadcast256_2xInt32,
		/// <summary>Broadcast 2 x <see cref="int"/> to 512 bits</summary>
		Broadcast512_2xInt32,
		/// <summary>Broadcast 2 x <c>bfloat16</c> to 128 bits</summary>
		Broadcast128_2xBFloat16,
		/// <summary>Broadcast 2 x <c>bfloat16</c> to 256 bits</summary>
		Broadcast256_2xBFloat16,
		/// <summary>Broadcast 2 x <c>bfloat16</c> to 512 bits</summary>
		Broadcast512_2xBFloat16,
	}
}
