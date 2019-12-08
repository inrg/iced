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

namespace Generator.InstructionInfo.Rust {
	[Generator(TargetLanguage.Rust, GeneratorNames.InstrInfoDicts)]
	sealed class RustDictGenerator {
		readonly IdentifierConverter idConverter;
		readonly GeneratorOptions generatorOptions;

		public RustDictGenerator(GeneratorOptions generatorOptions) {
			idConverter = RustIdentifierConverter.Create();
			this.generatorOptions = generatorOptions;
		}

		public void Generate() {
			var filename = Path.Combine(generatorOptions.RustDir, "info", "tests", "test_parser.rs");
			new FileUpdater(TargetLanguage.Rust, "OpAccessDict", filename).Generate(writer => WriteDict(writer, DictConstants.OpAccessConstants));
		}

		void WriteDict(FileWriter writer, (string name, EnumValue value)[] constants) {
			var opAccessTypeStr = OpAccessEnum.Instance.Name(idConverter);
			writer.WriteLine($"let mut to_access: HashMap<&'static str, {opAccessTypeStr}> = HashMap::new();");
			foreach (var constant in constants)
				writer.WriteLine($"let _ = to_access.insert(\"{constant.name}\", {opAccessTypeStr}::{constant.value.Name(idConverter)});");
		}
	}
}