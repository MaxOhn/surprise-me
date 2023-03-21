use std::marker::PhantomData;

use rand::{prelude::Distribution, Rng};

use crate::Surprise;

use super::UnitSurprise;

/// The surprise factor of [`PhantomData`]
pub type PhantomDataSurprise = UnitSurprise;

impl<T> Surprise for PhantomData<T> {
    type Factor = PhantomDataSurprise;
}

impl<T> Distribution<PhantomData<T>> for PhantomDataSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> PhantomData<T> {
        PhantomData
    }
}
