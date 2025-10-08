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
