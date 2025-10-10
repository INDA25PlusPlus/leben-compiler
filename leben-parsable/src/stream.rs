use crate::parser::Parsable;

pub struct ScopedStream<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> ScopedStream<'a> {
    pub fn new(buffer: &'a [u8]) -> ScopedStream<'a> {
        ScopedStream { buffer, index: 0 }
    }

    pub fn at_end(&self) -> bool {
        self.index == self.buffer.len()
    }

    pub fn read<'c>(
        &'c mut self,
        len: usize,
        pred: impl FnOnce(&'c [u8]) -> bool)
        -> Option<&'c [u8]> 
    {
        if self.buffer.len() < len + self.index { return None; }

        let requested_slice = &self.buffer[self.index..(self.index + len)];

        pred(requested_slice).then_some({
            self.index += len;
            requested_slice
        })
    }

    pub fn scope<T: Parsable<'a>>(
        &mut self, 
        parse_fn: impl for<'b> FnOnce(&'b mut ScopedStream) -> Option<T>) 
        -> Option<T>
    {
        let start_index = self.index;

        let result = parse_fn(self);

        if matches!(result, Some(..)) {
            #[cfg(feature = "leben_parsable_debug")] {
                println!("DEBUG BUFFER:\n{}$EOS", String::from_utf8_lossy(&self.buffer[self.index..]));
            }
        } else {
            self.index = start_index;
        }
        result
    }

    pub fn scope_with_span<T: Parsable<'a>>(
        &mut self, 
        parse_fn: impl for<'b> FnOnce(&'b mut ScopedStream) -> Option<T>) 
        -> Option<(T, &'a [u8])>
    {
        let start_index = self.index;

        let result = parse_fn(self);

        if let Some(result) = result {
            #[cfg(feature = "leben_parsable_debug")] {
                println!("DEBUG BUFFER:\n{}$EOS", String::from_utf8_lossy(&self.buffer[self.index..]));
            }
            Some((result, &self.buffer[start_index..self.index]))
        } else {
            self.index = start_index;
            None
        }
    }
}
