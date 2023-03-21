use rand::prelude::Distribution;

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of [`Result`]
pub struct ResultSurprise<T: Surprise, E: Surprise> {
    /// Probability for `Ok`
    pub chance: f64,
    /// The surprise factor for the Ok type
    pub ok: SurpriseFactor<T>,
    /// The surprise factor for the Err type
    pub err: SurpriseFactor<E>,
}

impl<T: Surprise, E: Surprise> Surprise for Result<T, E> {
    type Factor = ResultSurprise<T, E>;
}

impl<T: Surprise, E: Surprise> Distribution<Result<T, E>> for ResultSurprise<T, E> {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Result<T, E> {
        if rng.gen_bool(self.chance) {
            Ok(rng.sample(&self.ok))
        } else {
            Err(rng.sample(&self.err))
        }
    }
}
