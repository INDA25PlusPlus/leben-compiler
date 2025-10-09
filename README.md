# leben-compiler

## Grammar

The language's grammar in BNF format can be found in `./grammar.txt`

## Usage

### Parsing

To parse a source file and output its abstract syntax tree to a file in RON format, run:

`EXE <input> <output>`

Example: `cargo run -- examples/test.txt examples/out.ron`
