use std::{
    borrow::Cow,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use rand::{prelude::Distribution, Rng};

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of [`Cow`].
///
/// Always generates an owned version.
pub struct CowSurprise<T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
{
    // The surprise factor of the owned type
    pub inner: SurpriseFactor<<T as ToOwned>::Owned>,
}

impl<'a, T> Surprise for Cow<'a, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
{
    type Factor = CowSurprise<T>;
}

impl<'a, T> Distribution<Cow<'a, T>> for CowSurprise<T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cow<'a, T> {
        Cow::Owned(<<T as ToOwned>::Owned as Surprise>::generate_with_factor(
            rng,
            &self.inner,
        ))
    }
}

impl<T> Default for CowSurprise<T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
    SurpriseFactor<<T as ToOwned>::Owned>: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> Clone for CowSurprise<T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
    SurpriseFactor<<T as ToOwned>::Owned>: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Debug for CowSurprise<T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
    SurpriseFactor<<T as ToOwned>::Owned>: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("CowSurprise")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> PartialEq for CowSurprise<T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Surprise,
    SurpriseFactor<<T as ToOwned>::Owned>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
