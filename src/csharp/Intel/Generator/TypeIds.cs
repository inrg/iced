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

namespace Generator {
	static class TypeIds {
		public static readonly TypeId IcedConstants = new TypeId(nameof(IcedConstants));
		public static readonly TypeId DecoderTestParserConstants = new TypeId(nameof(DecoderTestParserConstants));
		public static readonly TypeId DecoderConstants = new TypeId(nameof(DecoderConstants));

		public static readonly TypeId Code = new TypeId(nameof(Code));
		public static readonly TypeId CodeSize = new TypeId(nameof(CodeSize));
		public static readonly TypeId CpuidFeature = new TypeId(nameof(CpuidFeature));
		public static readonly TypeId CpuidFeatureInternal = new TypeId(nameof(CpuidFeatureInternal));
		public static readonly TypeId DecoderOptions = new TypeId(nameof(DecoderOptions));
		public static readonly TypeId EvexOpCodeHandlerKind = new TypeId(nameof(EvexOpCodeHandlerKind));
		public static readonly TypeId HandlerFlags = new TypeId(nameof(HandlerFlags));
		public static readonly TypeId LegacyHandlerFlags = new TypeId(nameof(LegacyHandlerFlags));
		public static readonly TypeId MemorySize = new TypeId(nameof(MemorySize));
		public static readonly TypeId OpCodeHandlerKind = new TypeId(nameof(OpCodeHandlerKind));
		public static readonly TypeId PseudoOpsKind = new TypeId(nameof(PseudoOpsKind));
		public static readonly TypeId Register = new TypeId(nameof(Register));
		public static readonly TypeId SerializedDataKind = new TypeId(nameof(SerializedDataKind));
		public static readonly TypeId TupleType = new TypeId(nameof(TupleType));
		public static readonly TypeId VexOpCodeHandlerKind = new TypeId(nameof(VexOpCodeHandlerKind));
		public static readonly TypeId Mnemonic = new TypeId(nameof(Mnemonic));
		public static readonly TypeId GasCtorKind = new TypeId(nameof(GasCtorKind));
		public static readonly TypeId IntelCtorKind = new TypeId(nameof(IntelCtorKind));
		public static readonly TypeId MasmCtorKind = new TypeId(nameof(MasmCtorKind));
		public static readonly TypeId NasmCtorKind = new TypeId(nameof(NasmCtorKind));
		public static readonly TypeId GasSizeOverride = new TypeId(nameof(GasSizeOverride));
		public static readonly TypeId GasInstrOpInfoFlags = new TypeId(nameof(GasInstrOpInfoFlags));
		public static readonly TypeId IntelSizeOverride = new TypeId(nameof(IntelSizeOverride));
		public static readonly TypeId IntelBranchSizeInfo = new TypeId(nameof(IntelBranchSizeInfo));
		public static readonly TypeId IntelInstrOpInfoFlags = new TypeId(nameof(IntelInstrOpInfoFlags));
		public static readonly TypeId MasmInstrOpInfoFlags = new TypeId(nameof(MasmInstrOpInfoFlags));
		public static readonly TypeId NasmSignExtendInfo = new TypeId(nameof(NasmSignExtendInfo));
		public static readonly TypeId NasmSizeOverride = new TypeId(nameof(NasmSizeOverride));
		public static readonly TypeId NasmBranchSizeInfo = new TypeId(nameof(NasmBranchSizeInfo));
		public static readonly TypeId NasmInstrOpInfoFlags = new TypeId(nameof(NasmInstrOpInfoFlags));
		public static readonly TypeId RoundingControl = new TypeId(nameof(RoundingControl));
		public static readonly TypeId OpKind = new TypeId(nameof(OpKind));
		public static readonly TypeId Instruction_MemoryFlags = new TypeId(nameof(Instruction_MemoryFlags));
		public static readonly TypeId Instruction_OpKindFlags = new TypeId(nameof(Instruction_OpKindFlags));
		public static readonly TypeId Instruction_CodeFlags = new TypeId(nameof(Instruction_CodeFlags));
		public static readonly TypeId VectorLength = new TypeId(nameof(VectorLength));
		public static readonly TypeId MandatoryPrefixByte = new TypeId(nameof(MandatoryPrefixByte));
		public static readonly TypeId StateFlags = new TypeId(nameof(StateFlags));
		public static readonly TypeId EncodingKind = new TypeId(nameof(EncodingKind));
		public static readonly TypeId FlowControl = new TypeId(nameof(FlowControl));
		public static readonly TypeId OpCodeOperandKind = new TypeId(nameof(OpCodeOperandKind));
	}
}