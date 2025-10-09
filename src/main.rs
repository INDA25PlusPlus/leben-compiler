fn main() {
    println!("Hello, world!");
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
        end: EndOfStream,
    }

    #[test]
    fn test_derive() {
        assert_eq!(
            Some(VariableDeclaration::default()),
            VariableDeclaration::parse(&mut ScopedStream::new(b"test =wow"))
        );

        let parsed = VariableDeclaration::parse(&mut ScopedStream::new(b"test =wow"));

        println!("{}", ron::ser::to_string_pretty(&parsed.unwrap(), ron::ser::PrettyConfig::default()).unwrap());
    }
}
