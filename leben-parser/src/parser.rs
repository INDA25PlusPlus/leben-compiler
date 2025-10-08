use crate::stream::ScopedStream;

pub trait Parsable<'a>: Sized {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithSpan<'a, T: Parsable<'a>> {
    pub node: T,
    pub span: &'a [u8],
}

impl<'a, T> Parsable<'a> for WithSpan<'a, T>
where
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<WithSpan<'a, T>>
    {
        stream
            .scope_with_span(|stream| T::parse(stream))
            .map(|(node, span)| WithSpan { node, span })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Repeat<T, const MIN: usize>
where
    T: for<'a> Parsable<'a>
{
    pub nodes: Vec<T>,
}

impl<'a, T, const MIN: usize> Parsable<'a> for Repeat<T, MIN>
where 
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let mut nodes = Vec::new();
        while let Some(node) = stream
            .scope(|stream| T::parse(stream))
        {
            nodes.push(node);
        }
        (nodes.len() >= MIN).then_some(Repeat { nodes })
    }
}

pub type ZeroPlus<T> = Repeat<T, 0>;

pub type OnePlus<T> = Repeat<T, 1>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepeatLimited<T, const MIN: usize, const MAX: usize>
where
    T: for<'a> Parsable<'a>
{
    pub nodes: [Option<T>; MAX],
}

impl<'a, T, const MIN: usize, const MAX: usize> Parsable<'a> for RepeatLimited<T, MIN, MAX>
where 
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let mut nodes = [const { None }; MAX];
        for i in 0..MAX {
            let node = stream
                .scope(|stream| T::parse(stream));
            if matches!(node, None) {
                return (i >= MIN).then_some(RepeatLimited { nodes });
            }
            nodes[i] = node;
        }
        Some(RepeatLimited { nodes })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharLiteral<const CHAR: u8>;

impl<'a, const CHAR: u8> Parsable<'a> for CharLiteral<CHAR> {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream.scope(|stream| {
            stream.read(1, |slice| slice[0] == CHAR)
                .map(|_| CharLiteral)
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharRange<const START: u8, const END: u8>;

impl<'a, const START: u8, const END: u8> Parsable<'a> for CharRange<START, END> {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream.scope(|stream| {
            stream.read(1, |slice| (START..=END).contains(&slice[0]))
                .map(|_| CharRange)
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EndOfStream;

impl<'a> Parsable<'a> for EndOfStream {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream.at_end().then_some(EndOfStream)
    }
}

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

impl <'a> Parsable<'a> for () {
    fn parse(_: &mut ScopedStream<'a>) -> Option<Self> {
        Some(())
    }
}

pub fn parse_literal<'a>(stream: &mut ScopedStream<'a>, literal: &'static [u8]) -> Option<()> {
    stream.scope(|stream| {
        stream.read(literal.len(), |slice| slice == literal)
            .map(|_| ())
    })
}
