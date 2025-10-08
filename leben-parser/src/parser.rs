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
