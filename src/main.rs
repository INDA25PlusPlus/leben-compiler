use std::error::Error;

use leben_parsable::{CharLiteral, Parsable, ScopedStream, format_error_stack};

mod parsing;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 { panic!("Usage: EXE <input> <output>") }
    
    let input_path = std::path::Path::new(&args[1]);
    let buffer = std::fs::read(input_path)?;
    
    let mut stream = ScopedStream::new(&buffer);
    let parsed = parsing::SourceFile::parse(&mut stream).expect("failed to parse");

    match parsed {
        Ok(parsed) => {
            let config = ron::ser::PrettyConfig::default();
            let output = ron::ser::to_string_pretty(&parsed, config).unwrap();
            let output_path = std::path::Path::new(&args[2]);
            std::fs::write(output_path, output)?;
        },
        Err(err) => {
            eprintln!("{}", format_error_stack(&buffer, err));
        },
    }

    Ok(())
}
