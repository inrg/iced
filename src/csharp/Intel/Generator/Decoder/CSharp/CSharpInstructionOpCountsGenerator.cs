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

using System.IO;
using Generator.Constants;
using Generator.IO;

namespace Generator.Decoder.CSharp {
	[Generator(TargetLanguage.CSharp, GeneratorNames.Code_OpCount)]
	sealed class CSharpInstructionOpCountsGenerator {
		readonly IdentifierConverter idConverter;
		readonly GeneratorOptions generatorOptions;

		public CSharpInstructionOpCountsGenerator(GeneratorOptions generatorOptions) {
			idConverter = CSharpIdentifierConverter.Create();
			this.generatorOptions = generatorOptions;
		}

		public void Generate() {
			var data = InstructionOpCountsTable.Table;
			const string ClassName = "InstructionOpCounts";
			using (var writer = new FileWriter(TargetLanguage.CSharp, FileUtils.OpenWrite(Path.Combine(CSharpConstants.GetDirectory(generatorOptions, CSharpConstants.IcedNamespace), ClassName + ".g.cs")))) {
				writer.WriteFileHeader();

				writer.WriteLine($"namespace {CSharpConstants.IcedNamespace} {{");
				writer.Indent();
				writer.WriteLine($"static class {ClassName} {{");
				writer.Indent();

				writer.WriteLine($"internal static readonly byte[] OpCount = new byte[{IcedConstantsType.Instance.Name(idConverter)}.{IcedConstantsType.Instance["NumberOfCodeValues"].Name(idConverter)}] {{");
				writer.Indent();
				foreach (var d in data)
					writer.WriteLine($"{d.count},// {d.codeEnum.Name(idConverter)}");
				writer.Unindent();
				writer.WriteLine("};");
				writer.Unindent();
				writer.WriteLine("}");
				writer.Unindent();
				writer.WriteLine("}");
			}
		}
	}
}