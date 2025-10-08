use crate::stream::ScopedStream;

pub trait Parsable<'a>: Sized {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self>;
}

pub struct WithSpan<'a, T: Parsable<'a>> {
    node: T,
    span: &'a [u8],
}

impl<'a, T> Parsable<'a> for WithSpan<'a, T>
where
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<WithSpan<'a, T>>
    {
        stream
            .scope(|stream| T::parse(stream))
            .map(|(node, span)| WithSpan { node, span })
    }
}

pub struct Repeat<T, const MIN: usize>
where
    T: for<'a> Parsable<'a>
{
    nodes: Vec<T>,
}

impl<'a, T, const MIN: usize> Parsable<'a> for Repeat<T, MIN>
where 
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let mut nodes = Vec::new();
        while let Some(node) = stream
            .scope(|stream| T::parse(stream))
            .map(|(node, _)| node)
        {
            nodes.push(node);
        }
        (nodes.len() >= MIN).then_some(Repeat { nodes })
    }
}

pub type ZeroPlus<T> = Repeat<T, 0>;

pub type OnePlus<T> = Repeat<T, 1>;

pub struct RepeatLimited<T, const MIN: usize, const MAX: usize>
where
    T: for<'a> Parsable<'a>
{
    nodes: [Option<T>; MAX],
}

impl<'a, T, const MIN: usize, const MAX: usize> Parsable<'a> for RepeatLimited<T, MIN, MAX>
where 
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        let mut nodes = [const { None }; MAX];
        for i in 0..MAX {
            let node = stream
                .scope(|stream| T::parse(stream))
                .map(|(node, _)| node);
            if matches!(node, None) {
                return (i >= MIN).then_some(RepeatLimited { nodes });
            }
            nodes[i] = node;
        }
        Some(RepeatLimited { nodes })
    }
}

pub struct CharLiteral<const CHAR: u8>;

impl<'a, const CHAR: u8> Parsable<'a> for CharLiteral<CHAR> {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream.scope(|stream| {
            stream.read(1, |slice| slice[0] == CHAR)
                .map(|_| CharLiteral)
        }).map(|(ch, _)| ch)
    }
}

pub struct CharRange<const START: u8, const END: u8>;

impl<'a, const START: u8, const END: u8> Parsable<'a> for CharRange<START, END> {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream.scope(|stream| {
            stream.read(1, |slice| (START..=END).contains(&slice[0]))
                .map(|_| CharRange)
        }).map(|(ch, _)| ch)
    }
}

impl<'a, T> Parsable<'a> for Box<T>
where
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        stream
            .scope(|stream| T::parse(stream))
            .map(|(node, _)| Box::new(node))
    }
}

impl<'a, T> Parsable<'a> for Option<T>
where 
    T: for<'b> Parsable<'b>
{
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        Some(stream
            .scope(|stream| T::parse(stream))
            .map(|(node, _)| node))
    }
}

impl <'a> Parsable<'a> for () {
    fn parse(stream: &mut ScopedStream<'a>) -> Option<Self> {
        Some(())
    }
}

pub fn parse_literal<'a>(stream: &mut ScopedStream<'a>, literal: &'static [u8]) -> Option<()> {
    stream.scope(|stream| {
        stream.read(literal.len(), |slice| slice == literal)
            .map(|_| ())
    }).map(|_| ())
}
