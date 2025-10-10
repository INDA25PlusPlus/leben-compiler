# leben-parsable

## Usage

Use the `leben_parsable_debug` flag to print debug messages when parsing using the types provided in this crate.

Additionally, the `leben_parsable_derive_debug` config flag can be used to add debug messages to derived `Parsable` implementations.

Example debugging commands (powershell):

`clear; $env:RUSTFLAGS='--cfg leben_parsable_derive_debug'; cargo run --features leben-parsable/leben_parsable_debug -- examples/expr1.txt examples/out.ron; $env:RUSTFLAGS=''`
