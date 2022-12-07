#[derive(Debug)]
pub struct Slice<'a> {
    pub source: &'a [u8],
    pub beginning: usize,
    pub end: usize,
}

impl<'a> Slice<'_> {
    pub fn new(source: &'a [u8], beginning: usize, end: usize) -> Slice<'a> {
        return Slice {
            source,
            beginning,
            end,
        };
    }

    pub fn as_str(&self) -> &str {
        // Source bytes are extracted from an UTF-8 &str initially
        unsafe { std::str::from_utf8_unchecked(&self.source[self.beginning..self.end]) }
    }
}