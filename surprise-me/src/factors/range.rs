use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use rand::{prelude::Distribution, Rng};

use crate::Surprise;

use super::{NumberSurprise, UnitSurprise};

/// The surprise factor of [`Range`]
pub struct RangeSurprise<N> {
    /// The surprise factor of the range's start value
    pub start: NumberSurprise<N>,
    /// The surprise factor of the range's end value
    pub end: NumberSurprise<N>,
}

impl<N> Surprise for Range<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    type Factor = RangeSurprise<N>;
}

impl<N> Distribution<Range<N>> for RangeSurprise<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Range<N> {
        self.start.sample(rng)..self.end.sample(rng)
    }
}

/// The surprise factor of [`RangeInclusive`]
pub type RangeInclusiveSurprise<N> = RangeSurprise<N>;

impl<N> Surprise for RangeInclusive<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    type Factor = RangeInclusiveSurprise<N>;
}

impl<N> Distribution<RangeInclusive<N>> for RangeInclusiveSurprise<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangeInclusive<N> {
        self.start.sample(rng)..=self.end.sample(rng)
    }
}

/// The surprise factor of [`RangeFull`]
pub type RangeFullSurprise = UnitSurprise;

impl Surprise for RangeFull {
    type Factor = RangeFullSurprise;
}

impl Distribution<RangeFull> for RangeFullSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> RangeFull {
        ..
    }
}

/// The surprise factor of [`RangeFrom`]
pub type RangeFromSurprise<N> = NumberSurprise<N>;

impl<N> Surprise for RangeFrom<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    type Factor = RangeFromSurprise<N>;
}

impl<N> Distribution<RangeFrom<N>> for RangeFromSurprise<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangeFrom<N> {
        rng.sample::<N, _>(self)..
    }
}

/// The surprise factor of [`RangeTo`]
pub type RangeToSurprise<N> = NumberSurprise<N>;

impl<N> Surprise for RangeTo<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    type Factor = RangeToSurprise<N>;
}

impl<N> Distribution<RangeTo<N>> for RangeToSurprise<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangeTo<N> {
        ..rng.sample::<N, _>(self)
    }
}

/// The surprise factor of [`RangeToInclusive`]
pub type RangeToInclusiveSurprise<N> = NumberSurprise<N>;

impl<N> Surprise for RangeToInclusive<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    type Factor = RangeToInclusiveSurprise<N>;
}

impl<N> Distribution<RangeToInclusive<N>> for RangeToInclusiveSurprise<N>
where
    NumberSurprise<N>: Distribution<N>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangeToInclusive<N> {
        ..=rng.sample::<N, _>(self)
    }
}
