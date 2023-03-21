use rand::{
    distributions::{Alphanumeric, Standard},
    prelude::Distribution,
    Rng,
};

use crate::Surprise;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
/// The surprise factor of the unit type `()`
pub struct UnitSurprise;

impl Surprise for () {
    type Factor = UnitSurprise;
}

impl Distribution<()> for UnitSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) {}
}

macro_rules! surprise_number {
    (INT: $( $ty:ident $(,)? )* ) => {
        surprise_number!(@SURPRISE: $($ty,)*);

        $(
            impl Default for NumberSurprise<$ty> {
                #[inline]
                fn default() -> Self {
                    Self {
                        min: $ty::MIN,
                        max: $ty::MAX,
                    }
                }
            }
        )*
    };
    (FLOAT: $( $ty:ident $(,)? )* ) => {
        surprise_number!(@SURPRISE: $($ty,)*);

        $(
            impl Default for NumberSurprise<$ty> {
                #[inline]
                fn default() -> Self {
                    Self {
                        min: 0.0,
                        max: 1.0,
                    }
                }
            }
        )*
    };
    (@SURPRISE: $( $ty:ident $(,)? )* ) => {
        $(
            impl Surprise for $ty {
                type Factor = NumberSurprise<Self>;
            }

            impl Distribution<$ty> for NumberSurprise<$ty> {
                #[inline]
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    rng.gen_range(self.min..=self.max)
                }
            }
        )*
    }
}

surprise_number!(INT: u8, u16, u32, u64, u128, usize);
surprise_number!(INT: i8, i16, i32, i64, i128, isize);
surprise_number!(FLOAT: f32, f64);

#[derive(Clone, Debug, PartialEq)]
/// The surprise factor of numbers
pub struct NumberSurprise<N> {
    /// The minimum value of generated numbers.
    ///
    /// For integers the default is `MIN`, for `f32` and `f64` it's `0.0`
    pub min: N,
    /// The maximum value of generated numbers
    ///
    /// For integers the default is `MAX`, for `f32` and `f64` it's `1.0`
    pub max: N,
}

impl Surprise for bool {
    type Factor = BoolSurprise;
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// The surprise factor of [`bool`]
pub struct BoolSurprise {
    /// Probability for `true`.
    ///
    /// Should be between `0.0` and `1.0`.
    pub chance: f64,
}

impl Distribution<bool> for BoolSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.gen_bool(self.chance)
    }
}

impl Default for BoolSurprise {
    #[inline]
    fn default() -> Self {
        Self { chance: 0.5 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The surprise factor of [`char`]
pub enum CharSurprise {
    /// Only generates ASCII characters
    Ascii,
    /// Generates UTF-8 characters
    Unicode,
}

impl Default for CharSurprise {
    #[inline]
    fn default() -> Self {
        Self::Ascii
    }
}

impl Surprise for char {
    type Factor = CharSurprise;
}

impl Distribution<char> for CharSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        match self {
            Self::Ascii => rng.sample(Alphanumeric) as char,
            Self::Unicode => rng.sample(Standard),
        }
    }
}
