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

/// LEByteEncodedValue represents an arbitrarily length binary value encoded
/// in little-endian format
#[derive(Debug, Clone, PartialEq)]
pub struct LEByteEncodedValue {
    inner: Vec<u8>,
}

impl LEByteEncodedValue {
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[allow(dead_code)]
    /// leading_zeros returns the leading zeroes for the concrete type of the
    /// object. For example, 255u8 returns 0, 255u16 would return 8 when
    /// encoded as an LEByteEncodedValue much like their corresponding unsigned
    /// integer type.
    pub fn leading_zeros(&self) -> usize {
        self.inner
            .iter()
            .rev()
            .map(|b| b.leading_zeros())
            .fold(0usize, |acc, x| {
                if (acc % 8) == 0 {
                    acc + x as usize
                } else {
                    acc
                }
            })
    }
    /// bits outputs the number of bits needed to express a value.
    pub fn bits(&self) -> usize {
        self.inner
            .iter()
            .filter(|x| x.leading_zeros() < 8)
            .fold(0usize, |acc, &x| acc + (8 - x.leading_zeros()) as usize)
    }

    /// Returns the value as a little-endian encoded Vec<u8>
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }
}

impl crate::addressing::SizeOf for LEByteEncodedValue {
    fn size_of(&self) -> usize {
        self.len()
    }
}

impl crate::Emitter<Vec<u8>> for LEByteEncodedValue {
    fn emit(&self) -> Vec<u8> {
        self.inner.clone().into_iter().collect()
    }
}

macro_rules! impl_from_to_le_bytes {
    ($($t:ty,)*) => {
        $(
            impl From<$t> for LEByteEncodedValue {
                fn from(src: $t) -> Self {
                    Self { inner: src.to_le_bytes().to_vec() }
                }
            }
        )*
    };
}

impl_from_to_le_bytes!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_bits_required_to_express_a_type() {
        assert_eq!(3, super::LEByteEncodedValue::from(4u8).bits());
        assert_eq!(8, super::LEByteEncodedValue::from(255u8).bits());

        assert_eq!(3, super::LEByteEncodedValue::from(4u16).bits());
        assert_eq!(8, super::LEByteEncodedValue::from(255u16).bits());

        assert_eq!(3, super::LEByteEncodedValue::from(4u32).bits());
        assert_eq!(8, super::LEByteEncodedValue::from(255u32).bits());
    }

    #[test]
    fn should_return_leading_zeros_for_underlying_type() {
        assert_eq!(5, super::LEByteEncodedValue::from(4u8).leading_zeros());
        assert_eq!(0, super::LEByteEncodedValue::from(255u8).leading_zeros());

        assert_eq!(13, super::LEByteEncodedValue::from(4u16).leading_zeros());
        assert_eq!(8, super::LEByteEncodedValue::from(255u16).leading_zeros());

        assert_eq!(29, super::LEByteEncodedValue::from(4u32).leading_zeros());
        assert_eq!(24, super::LEByteEncodedValue::from(255u32).leading_zeros());
    }
}
