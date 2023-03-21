use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    rc::Rc,
    sync::Arc,
};

use rand::{prelude::Distribution, Rng};

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of [`Box`]
pub struct BoxSurprise<T: Surprise> {
    /// The surprise factor for the inner type
    pub inner: SurpriseFactor<T>,
}

impl<T: Surprise> Surprise for Box<T> {
    type Factor = BoxSurprise<T>;
}

impl<T: Surprise> Distribution<Box<T>> for BoxSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<T> {
        Box::new(self.inner.sample(rng))
    }
}

impl<T> Default for BoxSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> Clone for BoxSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Debug for BoxSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BoxSurprise")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> PartialEq for BoxSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/// The surprise factor of [`Rc`]
pub type RcSurprise<T> = BoxSurprise<T>;

impl<T: Surprise + ?Sized> Surprise for Rc<T> {
    type Factor = RcSurprise<T>;
}

impl<T: Surprise> Distribution<Rc<T>> for RcSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rc<T> {
        Rc::new(self.inner.sample(rng))
    }
}

/// The surprise factor of [`Arc`]
pub type ArcSurprise<T> = BoxSurprise<T>;

impl<T: Surprise + ?Sized> Surprise for Arc<T> {
    type Factor = ArcSurprise<T>;
}

impl<T: Surprise> Distribution<Arc<T>> for ArcSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Arc<T> {
        Arc::new(self.inner.sample(rng))
    }
}
