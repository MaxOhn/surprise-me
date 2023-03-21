use std::sync::{Mutex, RwLock};

use rand::{prelude::Distribution, Rng};

use crate::Surprise;

use super::BoxSurprise;

/// The surprise factor of [`Mutex`]
pub type MutexSurprise<T> = BoxSurprise<T>;

impl<T: Surprise> Surprise for Mutex<T> {
    type Factor = MutexSurprise<T>;
}

impl<T: Surprise> Distribution<Mutex<T>> for MutexSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mutex<T> {
        Mutex::new(rng.sample(&self.inner))
    }
}

/// The surprise factor of [`RwLock`]
pub type RwLockSurprise<T> = BoxSurprise<T>;

impl<T: Surprise> Surprise for RwLock<T> {
    type Factor = RwLockSurprise<T>;
}

impl<T: Surprise> Distribution<RwLock<T>> for RwLockSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RwLock<T> {
        RwLock::new(rng.sample(&self.inner))
    }
}
