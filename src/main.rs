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
