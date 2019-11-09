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

using System;
using System.Linq;
using Generator.Enums;

namespace Generator.Decoder {
	interface IInstructionMemorySizesGenerator {
		void Generate((EnumValue codeEnum, EnumValue mem, EnumValue bcst)[] data);
	}

	sealed class InstructionMemorySizesGenerator {
		readonly ProjectDirs projectDirs;

		public InstructionMemorySizesGenerator(ProjectDirs projectDirs) => this.projectDirs = projectDirs;

		public void Generate() {
			var generators = new IInstructionMemorySizesGenerator[(int)TargetLanguage.Last] {
				new CSharp.CSharpInstructionMemorySizesGenerator(projectDirs),
				new Rust.RustInstructionMemorySizesGenerator(projectDirs),
			};

			var data = InstructionMemorySizesTable.Table;
			if (data.Select(a => a.codeEnum).ToHashSet<EnumValue>().Count != CodeEnum.Instance.Values.Length)
				throw new InvalidOperationException();

			foreach (var generator in generators)
				generator.Generate(data);
		}
	}
}
