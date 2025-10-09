use std::error::Error;

use leben_parsable::{Parsable, ScopedStream};

mod parsing;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 { panic!("Usage: EXE <input> <output>") }
    
    let input_path = std::path::Path::new(&args[1]);
    let buffer = std::fs::read(input_path)?;
    
    let mut stream = ScopedStream::new(&buffer);
    let parsed = parsing::SourceFile::parse(&mut stream).expect("failed to parse");
    
    let config = ron::ser::PrettyConfig::default();
    let formatted = ron::ser::to_string_pretty(&parsed, config).unwrap();
    
    let output_path = std::path::Path::new(&args[2]);
    std::fs::write(output_path, formatted)?;
    Ok(())
}

#[cfg(test)]
mod derive_tests {
    use super::*;
    use leben_parsable::*;
    use serde::{Deserialize, Serialize};

    type Space = CharLiteral<b' '>;
    type EqualSign = CharLiteral<b'='>;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Parsable, Deserialize, Serialize)]
    struct VariableDeclaration {
        #[literal = b"test"]
        ident: (),
        s1: Ignore<Option<Space>>,
        eq: EqualSign,
        s2: Ignore<Option<Space>>,
        #[literal = b"wow"]
        expr: (),
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Parsable, Deserialize, Serialize)]
    struct ReturnKeyword { #[literal = b"return"] s1: () }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Parsable, Deserialize, Serialize)]
    struct IfKeyword { #[literal = b"if"] s1: () }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Parsable, Deserialize, Serialize)]
    struct ElseKeyword { #[literal = b"else"] s1: () }

    #[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
    enum Keyword {
        Return(ReturnKeyword),
        If(IfKeyword),
        Else(ElseKeyword),
    }

    #[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
    struct Combined {
        var: VariableDeclaration,
        _0: CharLiteral<b';'>,
        _1: OnePlus<Space>,
        keyword: Keyword,
        end: EndOfStream,
    }

    #[test]
    fn struct_derive() {
        assert_eq!(
            Some(VariableDeclaration::default()),
            VariableDeclaration::parse(&mut ScopedStream::new(b"test =wow"))
        );

        let parsed = VariableDeclaration::parse(&mut ScopedStream::new(b"test =wow"));

        println!("{}", ron::ser::to_string_pretty(&parsed.unwrap(), ron::ser::PrettyConfig::default()).unwrap());
    }

    #[test]
    fn enum_derive() {
        assert_eq!(
            Some(Keyword::If(IfKeyword { s1: () })),
            Keyword::parse(&mut ScopedStream::new(b"if"))
        );

        let parsed = Combined::parse(&mut ScopedStream::new(b"test =wow; return"));

        println!("{}", ron::ser::to_string_pretty(&parsed.unwrap(), ron::ser::PrettyConfig::default()).unwrap());
    }
}
