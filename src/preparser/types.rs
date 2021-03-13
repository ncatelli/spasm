/// Type System errors.
#[derive(Clone, PartialEq)]
pub enum TypeError {
    IllegalType(String),
}

impl std::fmt::Debug for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalType(v) => write!(f, "illegal type: {:?}", v),
        }
    }
}

/// Reify functions to convert a type to a more concrete type.
pub trait Reify<T> {
    type Error;

    fn reify(&self) -> Result<T, Self::Error>;
}

/// BitValue represents an arbitrarily length binary value encoded in little-endian format
#[derive(Debug, Clone, PartialEq)]
pub struct BitValue {
    inner: Vec<u8>,
}

impl BitValue {
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn leading_zeros(&self) -> usize {
        self.inner.last().map_or(8, |b| b.leading_zeros() as usize)
    }

    // Returns the value as a little-endian encoded Vec<u8>
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }

    /// bits outputs the number of bits needed to express a value.
    pub fn bits(&self) -> usize {
        let leading_bits = 8 - self.leading_zeros();
        // bytes in addition to the most significant byte
        let bytes = if self.inner.len() > 0 {
            self.inner.len() - 1
        } else {
            0
        };

        (bytes * 8) + leading_bits
    }
}

impl crate::addressing::SizeOf for BitValue {
    fn size_of(&self) -> usize {
        self.len()
    }
}

impl crate::Emitter<Vec<u8>> for BitValue {
    fn emit(&self) -> Vec<u8> {
        self.inner.clone().into_iter().collect()
    }
}

macro_rules! impl_from_to_le_bytes {
    ($($t:ty,)*) => {
        $(
            impl From<$t> for BitValue {
                fn from(src: $t) -> Self {
                    Self { inner: src.to_le_bytes().to_vec() }
                }
            }
        )*
    };
}

impl_from_to_le_bytes!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);
