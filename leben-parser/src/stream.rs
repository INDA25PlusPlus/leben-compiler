use crate::parser::Parsable;

pub struct ScopedStream<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> ScopedStream<'a> {
    pub fn new(buffer: &'a [u8]) -> ScopedStream<'a> {
        ScopedStream { buffer, index: 0 }
    }

    pub fn read(
        &'a mut self, 
        len: usize,
        read_fn: impl FnOnce(&'a [u8]) -> bool)
        -> Option<&'a [u8]> 
    {
        if self.buffer.len() + self.index < len { return None; }

        let requested_slice = &self.buffer[self.index..(self.index + len)];

        read_fn(requested_slice).then_some({
            self.index += len;
            requested_slice
        })
    }

    pub fn scope<T: Parsable<'a>>(
        &mut self, 
        parse_fn: impl for<'b> FnOnce(&'b mut ScopedStream) -> Option<T>) 
        -> Option<(T, &'a [u8])>
    {
        let start_index = self.index;

        let result = parse_fn(self);

        if let Some(result) = result {
            Some((result, &self.buffer[start_index..self.index]))
        } else {
            self.index = start_index;
            None
        }
    }
}
