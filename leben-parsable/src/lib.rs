pub mod stream;
pub mod parser;

pub use stream::*;
pub use parser::*;
pub use leben_parsable_derive::Parsable;

#[cfg(test)]
mod parse_tests {
    use crate::{parser::{parse_literal, CharLiteral, Parsable, Repeat, ZeroPlus}, stream::ScopedStream, Unparsable};
    use std::fmt::Debug;

    enum Variants {
        V1(CharLiteral<b'1'>),
        V2(CharLiteral<b'2'>),
    }

    impl<'p> Parsable<'p> for Variants {
        fn parse(stream: &mut ScopedStream<'p>) -> Option<Self> {
            stream.scope(|stream| {
                None
                    .or(<CharLiteral<b'1'> as Parsable>::parse(stream).map(|v| Variants::V1(v)))
                    .or(<CharLiteral<b'2'> as Parsable>::parse(stream).map(|v| Variants::V2(v)))
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Compound<A, B> {
        a: A,
        b: B,
    }

    impl<'p, A, B> Parsable<'p> for Compound<A, B>
    where
        A: for<'a> Parsable<'a>,
        B: for<'b> Parsable<'b>,
    {
        fn parse(stream: &mut ScopedStream<'p>) -> Option<Self> {
            stream.scope(|stream| {
                Some(Self {
                    a: <A as Parsable<'_>>::parse(stream)?,
                    b: <B as Parsable<'_>>::parse(stream)?,
                })
            })
        }
    }

    fn test_string<'a, T: Parsable<'a> + Debug + PartialEq>(string: &'static [u8], expected: Option<T>) {
        assert_eq!(
            expected,
            T::parse(&mut ScopedStream::new(string)),
        );
    }

    #[test]
    fn char_literal() {
        test_string::<CharLiteral<b'a'>>(b"a", Some(CharLiteral));
        assert_eq!(Some(()), parse_literal(&mut ScopedStream::new(b"test"), b"test"));
        assert_eq!(None, parse_literal(&mut ScopedStream::new(b"test"), b"ahah"));

        type T = Compound<Repeat<CharLiteral<b'a'>, 3>, ZeroPlus<CharLiteral<b'b'>>>;

        assert_eq!(Some(Compound { a: Repeat { nodes: vec![CharLiteral; 4] }, b: Repeat { nodes: vec![CharLiteral; 0] } }), 
                   T::parse(&mut ScopedStream::new(b"aaaa")));
        assert_eq!(Some(Compound { a: Repeat { nodes: vec![CharLiteral; 3] }, b: Repeat { nodes: vec![CharLiteral; 5]} }),
                   T::parse(&mut ScopedStream::new(b"aaabbbbb")));
        assert_eq!(Some(Compound { a: Repeat { nodes: vec![CharLiteral; 8] }, b: Repeat { nodes: vec![CharLiteral; 1]} }),
                   T::parse(&mut ScopedStream::new(b"aaaaaaaab")));
        assert_eq!(None, T::parse(&mut ScopedStream::new(b"aabbb")));
        assert_eq!(None, T::parse(&mut ScopedStream::new(b"bb")));
    }
}
