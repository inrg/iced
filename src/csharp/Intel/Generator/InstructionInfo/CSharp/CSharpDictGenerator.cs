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
using Generator.Enums;
using Generator.Enums.InstructionInfo;
using Generator.IO;

namespace Generator.InstructionInfo.CSharp {
	[Generator(TargetLanguage.CSharp, GeneratorNames.InstrInfoDicts)]
	sealed class CSharpDictGenerator {
		readonly IdentifierConverter idConverter;
		readonly GeneratorOptions generatorOptions;

		public CSharpDictGenerator(GeneratorOptions generatorOptions) {
			idConverter = CSharpIdentifierConverter.Create();
			this.generatorOptions = generatorOptions;
		}

		public void Generate() {
			var filename = Path.Combine(generatorOptions.CSharpTestsDir, "Intel", "InstructionInfoTests", "InstructionInfoConstants.cs");
			new FileUpdater(TargetLanguage.CSharp, "Dicts", filename).Generate(writer => WriteDicts(writer));
		}

		void WriteDicts(FileWriter writer) =>
			WriteDict(writer, DictConstants.OpAccessConstants);

		void WriteDict(FileWriter writer, (string name, EnumValue value)[] constants) {
			var opAccessTypeStr = OpAccessEnum.Instance.Name(idConverter);
			writer.WriteLine($"internal static readonly Dictionary<string, {opAccessTypeStr}> ToAccess = new Dictionary<string, {opAccessTypeStr}>(StringComparer.Ordinal) {{");
			using (writer.Indent()) {
				foreach (var constant in constants)
					writer.WriteLine($"{{ \"{constant.name}\", {opAccessTypeStr}.{constant.value.Name(idConverter)} }},");
			}
			writer.WriteLine("};");
		}
	}
}