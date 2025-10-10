use std::marker::PhantomData;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

use crate::stream::ScopedStream;

pub trait Parsable<'a>: Sized {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self>;
}

#[derive(Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct WithSpan<T>
where
    T: for<'a> Parsable<'a>
{
    pub node: T,
    pub span: Vec<u8>,
}

impl<T> Debug for WithSpan<T>
where T: 
    for<'a> Parsable<'a> + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithSpan")
            .field("node", &self.node)
            .field("span", &String::from_utf8_lossy(&self.span))
            .finish()
    }
}

impl<'a, T> Parsable<'a> for WithSpan<T>
where
    T: for<'b> Parsable<'b> + Debug
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<WithSpan<T>>
    {
        let res = stream
            .scope_with_span(|stream| T::parse(stream))
            .map(|(node, span)| WithSpan { node, span: span.to_owned() });
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG SPAN        {:?}",
                res.as_ref().map(|span| String::from_utf8_lossy(&span.span)));
        }
        res
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Repeat<T, const MIN: usize>
where
    T: for<'a> Parsable<'a> + Debug
{
    pub nodes: Vec<T>,
}

impl<'a, T, const MIN: usize> Parsable<'a> for Repeat<T, MIN>
where 
    T: for<'b> Parsable<'b> + Debug
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG REPEAT >>>> {} * {}", std::any::type_name::<T>(), MIN);
        }
        let mut nodes = Vec::new();
        while let Some(node) = stream
            .scope(|stream| T::parse(stream))
        {
            nodes.push(node);
        }
        let res = (nodes.len() >= MIN).then_some(Repeat { nodes });
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG REPEAT <<<< {} * {}\n{:?}", std::any::type_name::<T>(), MIN, &res);
        }
        res
    }
}

pub type ZeroPlus<T> = Repeat<T, 0>;

pub type OnePlus<T> = Repeat<T, 1>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct RepeatLimited<T, const MIN: usize, const MAX: usize>
where
    T: for<'a> Parsable<'a>
{
    pub nodes: Vec<T>,
}

impl<'a, T, const MIN: usize, const MAX: usize> Parsable<'a> for RepeatLimited<T, MIN, MAX>
where 
    T: for<'b> Parsable<'b> + Debug
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG REPEATLIM > {} * ({}-{})", std::any::type_name::<T>(), MIN, MAX);
        }
        let mut nodes = Vec::with_capacity(MAX);
        for i in 0..MAX {
            let node = stream
                .scope(|stream| T::parse(stream));

            if let Some(node) = node {
                nodes.push(node);
            } else {
                let res = (i >= MIN).then_some(RepeatLimited { nodes });
                #[cfg(feature = "leben_parsable_debug")] {
                    println!("DEBUG REPEATLIM < {} * ({}-{})\n{:?}", std::any::type_name::<T>(), MIN, MAX, &res);
                }
                return res;
            }
        }
        let res = Some(RepeatLimited { nodes });
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG REPEATLIM < {} * ({}-{})\n{:?}", std::any::type_name::<T>(), MIN, MAX, &res);
        }
        res
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CharLiteral<const CHAR: u8>;

impl<'a, const CHAR: u8> Parsable<'a> for CharLiteral<CHAR> {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let res = stream.scope(|stream| {
            stream.read(1, |slice| slice[0] == CHAR)
                .map(|_| CharLiteral)
        });
        res
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CharRange<const START: u8, const END: u8>;

impl<'a, const START: u8, const END: u8> Parsable<'a> for CharRange<START, END> {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream.scope(|stream| {
            stream.read(1, |slice| (START..=END).contains(&slice[0]))
                .map(|_| CharRange)
        })
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EndOfStream;

impl<'a> Parsable<'a> for EndOfStream {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let res = stream.at_end().then_some(EndOfStream);
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG END         {:?}", &res);
        }
        res
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WithEnd<T>
where
    T: for<'a> Parsable<'a>
{
    pub node: T
}

impl<'a, T> Parsable<'a> for WithEnd<T>
where
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let res = stream
            .scope(|stream| T::parse(stream))
            .map(|node| WithEnd { node });
        if !stream.at_end() { return None };
        #[cfg(feature = "leben_parsable_debug")] {
            println!("DEBUG WITH END  < {}\n{:?}", std::any::type_name::<T>(), &res);
        }
        res
    }
}

#[derive(Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ignore<T>
where T: 
    for<'a> Parsable<'a> 
{
    #[serde(skip_serializing)]
    phantom_data: PhantomData<T>,
}

impl<T> Debug for Ignore<T>
where T: 
    for<'a> Parsable<'a> 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ignore").finish()
    }
}

impl<'a, T> Parsable<'a> for Ignore<T>
where T:
    for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream
            .scope(|stream| T::parse(stream))
            .map(|_| Ignore { phantom_data: PhantomData })
    }
}

pub type Span<T> = WithSpan<Ignore<T>>;

impl<'a, T> Parsable<'a> for Box<T>
where
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream
            .scope(|stream| T::parse(stream))
            .map(|node| Box::new(node))
    }
}

impl<'a, T> Parsable<'a> for Option<T>
where 
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        Some(stream
            .scope(|stream| T::parse(stream)))
    }
}

impl<'a> Parsable<'a> for () {
    fn parse(_: &mut ScopedStream<'a>) -> Option<Self> {
        Some(())
    }
}

pub fn parse_literal<'a>(stream: &mut ScopedStream<'a>, literal: &'static [u8]) -> Option<()> {
    let res = stream.scope(|stream| {
        stream.read(literal.len(), |slice| slice == literal)
            .map(|_| ())
    });
    #[cfg(feature = "leben_parsable_debug")] {
        let lit = String::from_utf8_lossy(literal);
        println!("DEBUG LITERAL     {} {:?}", &lit, res.as_ref().map(|_| &lit));
    }
    res
}

#[macro_export]
macro_rules! Literal {
    { $( $vis:vis struct $name:ident = $lit:literal; )* } => {
        $(
            #[derive(Parsable, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
            $vis struct $name {
                #[literal = $lit] _0: (),
            }

            impl std::fmt::Debug for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_tuple(stringify!($name)).field(&stringify!($lit)).finish()
                }
            }
        )*
    }

}
