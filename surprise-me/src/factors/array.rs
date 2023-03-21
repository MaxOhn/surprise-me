use std::fmt::{Debug, Formatter, Result as FmtResult};

use rand::prelude::Distribution;

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of arrays
pub struct ArraySurprise<T: Surprise, const N: usize> {
    /// The surprise factor of items
    pub items: SurpriseFactor<T>,
}

impl<T: Surprise, const N: usize> Surprise for [T; N] {
    type Factor = ArraySurprise<T, N>;
}

impl<T: Surprise, const N: usize> Distribution<[T; N]> for ArraySurprise<T, N> {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> [T; N] {
        [(); N].map(|_| self.items.sample(rng))
    }
}

impl<T, const N: usize> Default for ArraySurprise<T, N>
where
    T: Surprise,
    SurpriseFactor<T>: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}

impl<T, const N: usize> Clone for ArraySurprise<T, N>
where
    T: Surprise,
    SurpriseFactor<T>: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<T, const N: usize> Debug for ArraySurprise<T, N>
where
    T: Surprise,
    SurpriseFactor<T>: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ArraySurprise")
            .field("inner", &self.items)
            .finish()
    }
}

impl<T, const N: usize> PartialEq for ArraySurprise<T, N>
where
    T: Surprise,
    SurpriseFactor<T>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}
