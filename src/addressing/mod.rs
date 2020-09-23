/// SizeOf provides the trait to implement referencing the size of an object
// in bytes.
pub trait SizeOf {
    fn size_of(&self) -> usize {
        std::mem::size_of_val(&self)
    }
}

/// Positional functions to store an object wrapped with an offset position
pub struct Positional<T> {
    position: usize,
    contents: T,
}

impl<T> Positional<T> {
    /// new instantiates a Positional with an offset of 0 that wraps a value T.
    /// Essentially this calls Self::with_position(0, T).
    pub fn new(contents: T) -> Self {
        Self::with_position(0, contents)
    }

    /// with_position instantiates a new position, taking a starting offset and a contents to wrap.
    pub fn with_position(position: usize, contents: T) -> Self {
        Self { position, contents }
    }

    /// Unwraps the contents of a Positional returning the value it wraps.
    /// Transforming the type Positional<T> -> T.
    pub fn unwrap(self) -> T {
        self.contents
    }
}
