use std::fmt::{Debug, Formatter, Result as FmtResult};

use rand::{prelude::Distribution, Rng};

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of [`Option`]
pub struct OptionSurprise<T: Surprise> {
    /// Probability for `Some`.
    ///
    /// Should be between `0.0` and `1.0`.
    pub chance: f64,
    /// The surprise factor of the inner type
    pub inner: SurpriseFactor<T>,
}

impl<T: Surprise> Surprise for Option<T> {
    type Factor = OptionSurprise<T>;
}

impl<T: Surprise> Distribution<Option<T>> for OptionSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<T> {
        if rng.gen_bool(self.chance) {
            Some(T::generate_with_factor(rng, &self.inner))
        } else {
            None
        }
    }
}

impl<T> Default for OptionSurprise<T>
where
    T: Surprise,
    SurpriseFactor<T>: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            chance: 0.5,
            inner: Default::default(),
        }
    }
}

impl<T> Clone for OptionSurprise<T>
where
    T: Surprise,
    SurpriseFactor<T>: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            chance: self.chance,
            inner: self.inner.clone(),
        }
    }
}

impl<T> Debug for OptionSurprise<T>
where
    T: Surprise,
    SurpriseFactor<T>: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("OptionSurprise")
            .field("probability_some", &self.chance)
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> PartialEq for OptionSurprise<T>
where
    T: Surprise,
    SurpriseFactor<T>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.chance == other.chance && self.inner == other.inner
    }
}
