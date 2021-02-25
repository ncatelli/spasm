use super::typechecker;

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

/// Represents all variants of accepted types without their associated values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveType {
    Uint8,
    Uint16,
    Uint32,
}

impl From<PrimitiveVariant> for PrimitiveType {
    fn from(src: PrimitiveVariant) -> PrimitiveType {
        std::convert::From::from(&src)
    }
}

impl From<&PrimitiveVariant> for PrimitiveType {
    fn from(src: &PrimitiveVariant) -> PrimitiveType {
        match src {
            PrimitiveVariant::Uint8(_) => Self::Uint8,
            PrimitiveVariant::Uint16(_) => Self::Uint16,
            PrimitiveVariant::Uint32(_) => Self::Uint32,
        }
    }
}

/// Represents all variants of accepted types without their associated values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveType {
    Uint8,
    Uint16,
    Uint32,
}

impl From<PrimitiveVariant> for PrimitiveType {
    fn from(src: PrimitiveVariant) -> PrimitiveType {
        std::convert::From::from(&src)
    }
}

impl From<&PrimitiveVariant> for PrimitiveType {
    fn from(src: &PrimitiveVariant) -> PrimitiveType {
        match src {
            PrimitiveVariant::Uint8(_) => Self::Uint8,
            PrimitiveVariant::Uint16(_) => Self::Uint16,
            PrimitiveVariant::Uint32(_) => Self::Uint32,
        }
    }
}

/// A concrete type representing all valid type variants for the preparser.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveVariant {
    Uint8(Primitive<u8>),
    Uint16(Primitive<u16>),
    Uint32(Primitive<u32>),
}

impl crate::Emitter<Vec<u8>> for PrimitiveVariant {
    fn emit(&self) -> Vec<u8> {
        match *self {
            Self::Uint8(v) => v.unwrap().to_ne_bytes().to_vec(),
            Self::Uint16(v) => v.unwrap().to_ne_bytes().to_vec(),
            Self::Uint32(v) => v.unwrap().to_ne_bytes().to_vec(),
        }
    }
}

impl crate::addressing::SizeOf for PrimitiveVariant {
    fn size_of(&self) -> usize {
        match *self {
            Self::Uint8(_) => 1,
            Self::Uint16(_) => 2,
            Self::Uint32(_) => 4,
        }
    }
}

impl From<Primitive<u8>> for PrimitiveVariant {
    fn from(src: Primitive<u8>) -> Self {
        PrimitiveVariant::Uint8(src)
    }
}

impl From<Primitive<u16>> for PrimitiveVariant {
    fn from(src: Primitive<u16>) -> Self {
        PrimitiveVariant::Uint16(src)
    }
}

impl From<Primitive<u32>> for PrimitiveVariant {
    fn from(src: Primitive<u32>) -> Self {
        PrimitiveVariant::Uint32(src)
    }
}

impl std::convert::TryFrom<PrimitiveVariant> for Primitive<u8> {
    type Error = TypeError;

    fn try_from(src: PrimitiveVariant) -> Result<Self, Self::Error> {
        match src {
            PrimitiveVariant::Uint8(p) => Ok(p),
            PrimitiveVariant::Uint16(p) => Err(TypeError::IllegalType(p.to_string())),
            PrimitiveVariant::Uint32(p) => Err(TypeError::IllegalType(p.to_string())),
        }
    }
}

impl std::convert::TryFrom<PrimitiveVariant> for Primitive<u16> {
    type Error = TypeError;

    fn try_from(src: PrimitiveVariant) -> Result<Self, Self::Error> {
        match src {
            PrimitiveVariant::Uint8(p) => Ok(Primitive::new(u16::from(p.unwrap()))),
            PrimitiveVariant::Uint16(p) => Ok(p),
            PrimitiveVariant::Uint32(p) => Err(TypeError::IllegalType(p.to_string())),
        }
    }
}

impl std::convert::TryFrom<PrimitiveVariant> for Primitive<u32> {
    type Error = TypeError;

    fn try_from(src: PrimitiveVariant) -> Result<Self, Self::Error> {
        match src {
            PrimitiveVariant::Uint8(p) => Ok(Primitive::new(u32::from(p.unwrap()))),
            PrimitiveVariant::Uint16(p) => Ok(Primitive::new(u32::from(p.unwrap()))),
            PrimitiveVariant::Uint32(p) => Ok(p),
        }
    }
}

impl crate::preparser::typechecker::Kinded for PrimitiveVariant {
    fn kind(&self) -> typechecker::Kind {
        let pt: PrimitiveType = self.into();
        typechecker::Kind::from(pt)
    }
}

/// Primitive wraps rust primitive
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Primitive<T> {
    inner: T,
}

impl<T> Primitive<T> {
    /// Instantiates a new Primitive from an internal type.
    #[allow(dead_code)]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns the enclosed value of the primitive.
    #[allow(dead_code)]
    pub fn unwrap(self) -> T {
        self.inner
    }
}

impl<T> std::fmt::Display for Primitive<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::convert::From<Primitive<u8>> for Primitive<u16> {
    fn from(src: Primitive<u8>) -> Self {
        Primitive::new(u16::from(src.unwrap()))
    }
}

impl std::convert::From<Primitive<u8>> for Primitive<u32> {
    fn from(src: Primitive<u8>) -> Self {
        Primitive::new(u32::from(src.unwrap()))
    }
}

impl std::convert::From<Primitive<u16>> for Primitive<u32> {
    fn from(src: Primitive<u16>) -> Self {
        Primitive::new(u32::from(src.unwrap()))
    }
}

impl std::ops::Add for Primitive<u8> {
    type Output = Primitive<u8>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self.unwrap();
        let rhs = rhs.unwrap();
        let sum = lhs.overflowing_add(rhs).0;
        Primitive::new(sum)
    }
}

impl std::ops::Add for Primitive<u16> {
    type Output = Primitive<u16>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self.unwrap();
        let rhs = rhs.unwrap();
        let sum = lhs.overflowing_add(rhs).0;
        Primitive::new(sum)
    }
}

impl std::ops::Add for Primitive<u32> {
    type Output = Primitive<u32>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self.unwrap();
        let rhs = rhs.unwrap();
        let sum = lhs.overflowing_add(rhs).0;
        Primitive::new(sum)
    }
}

impl<T> crate::preparser::typechecker::Kinded for Primitive<T>
where
    T: Copy,
    Self: Into<PrimitiveVariant>,
{
    fn kind(&self) -> typechecker::Kind {
        let pt: PrimitiveType = Into::<PrimitiveVariant>::into(*self).into();
        typechecker::Kind::from(pt)
    }
}
