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

#if !NO_MASM_FORMATTER && !NO_FORMATTER
using System;

namespace Iced.Intel.MasmFormatterInternal {
	[Flags]
	enum InstrOpInfoFlags : ushort {
		None = 0x00000000,
		MemSize_Mask = 0x00000007,
		MemSize_Sse = 0x00000000,
		MemSize_Mmx = 0x00000001,
		MemSize_Normal = 0x00000002,
		MemSize_Nothing = 0x00000003,
		MemSize_XmmwordPtr = 0x00000004,
		MemSize_DwordOrQword = 0x00000005,
		ShowNoMemSize_ForceSize = 0x00000008,
		ShowMinMemSize_ForceSize = 0x00000010,
		JccNotTaken = 0x00000020,
		JccTaken = 0x00000040,
		BndPrefix = 0x00000080,
		IgnoreIndexReg = 0x00000100,
		MnemonicIsDirective = 0x00000200,
	}
}
#endif
