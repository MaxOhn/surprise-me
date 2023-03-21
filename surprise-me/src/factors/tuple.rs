use rand::{distributions::Distribution, Rng};

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of tuples
pub struct TupleSurprise<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>(
    pub SurpriseFactor<T1>,
    pub SurpriseFactor<T2>,
    pub SurpriseFactor<T3>,
    pub SurpriseFactor<T4>,
    pub SurpriseFactor<T5>,
    pub SurpriseFactor<T6>,
    pub SurpriseFactor<T7>,
    pub SurpriseFactor<T8>,
    pub SurpriseFactor<T9>,
    pub SurpriseFactor<T10>,
    pub SurpriseFactor<T11>,
    pub SurpriseFactor<T12>,
    pub SurpriseFactor<T13>,
    pub SurpriseFactor<T14>,
    pub SurpriseFactor<T15>,
    pub SurpriseFactor<T16>,
)
where
    T1: Surprise,
    T2: Surprise,
    T3: Surprise,
    T4: Surprise,
    T5: Surprise,
    T6: Surprise,
    T7: Surprise,
    T8: Surprise,
    T9: Surprise,
    T10: Surprise,
    T11: Surprise,
    T12: Surprise,
    T13: Surprise,
    T14: Surprise,
    T15: Surprise,
    T16: Surprise;

macro_rules! tuple_surprise {
    ( $( $tuple_ty:ident : $idx:tt ),+ $(..)? $( $remaining_ty:ty ),* ) => {
        impl<$( $tuple_ty: Surprise ,)*> Surprise for ( $( $tuple_ty ,)* ) {
            type Factor = TupleSurprise<$( $tuple_ty ,)* $( $remaining_ty ,)*>;
        }

        impl<$( $tuple_ty: Surprise ,)*> Distribution<( $( $tuple_ty ,)* )> for TupleSurprise<$( $tuple_ty ,)* $( $remaining_ty ),*> {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ( $( $tuple_ty ,)* ) {
                ( $( self.$idx.sample(rng), )* )
            }
        }
    };
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Default
    for TupleSurprise<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
where
    T1: Surprise,
    <T1 as Surprise>::Factor: Default,
    T2: Surprise,
    <T2 as Surprise>::Factor: Default,
    T3: Surprise,
    <T3 as Surprise>::Factor: Default,
    T4: Surprise,
    <T4 as Surprise>::Factor: Default,
    T5: Surprise,
    <T5 as Surprise>::Factor: Default,
    T6: Surprise,
    <T6 as Surprise>::Factor: Default,
    T7: Surprise,
    <T7 as Surprise>::Factor: Default,
    T8: Surprise,
    <T8 as Surprise>::Factor: Default,
    T9: Surprise,
    <T9 as Surprise>::Factor: Default,
    T10: Surprise,
    <T10 as Surprise>::Factor: Default,
    T11: Surprise,
    <T11 as Surprise>::Factor: Default,
    T12: Surprise,
    <T12 as Surprise>::Factor: Default,
    T13: Surprise,
    <T13 as Surprise>::Factor: Default,
    T14: Surprise,
    <T14 as Surprise>::Factor: Default,
    T15: Surprise,
    <T15 as Surprise>::Factor: Default,
    T16: Surprise,
    <T16 as Surprise>::Factor: Default,
{
    #[inline]
    fn default() -> Self {
        Self(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Clone
    for TupleSurprise<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
where
    T1: Surprise,
    <T1 as Surprise>::Factor: Clone,
    T2: Surprise,
    <T2 as Surprise>::Factor: Clone,
    T3: Surprise,
    <T3 as Surprise>::Factor: Clone,
    T4: Surprise,
    <T4 as Surprise>::Factor: Clone,
    T5: Surprise,
    <T5 as Surprise>::Factor: Clone,
    T6: Surprise,
    <T6 as Surprise>::Factor: Clone,
    T7: Surprise,
    <T7 as Surprise>::Factor: Clone,
    T8: Surprise,
    <T8 as Surprise>::Factor: Clone,
    T9: Surprise,
    <T9 as Surprise>::Factor: Clone,
    T10: Surprise,
    <T10 as Surprise>::Factor: Clone,
    T11: Surprise,
    <T11 as Surprise>::Factor: Clone,
    T12: Surprise,
    <T12 as Surprise>::Factor: Clone,
    T13: Surprise,
    <T13 as Surprise>::Factor: Clone,
    T14: Surprise,
    <T14 as Surprise>::Factor: Clone,
    T15: Surprise,
    <T15 as Surprise>::Factor: Clone,
    T16: Surprise,
    <T16 as Surprise>::Factor: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
            self.4.clone(),
            self.5.clone(),
            self.6.clone(),
            self.7.clone(),
            self.8.clone(),
            self.9.clone(),
            self.10.clone(),
            self.11.clone(),
            self.12.clone(),
            self.13.clone(),
            self.14.clone(),
            self.15.clone(),
        )
    }
}

tuple_surprise!(T1: 0 .. (), (), (), (), (), (), (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1 .. (), (), (), (), (), (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2 .. (), (), (), (), (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3 .. (), (), (), (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4 .. (), (), (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5 .. (), (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6 .. (), (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7 .. (), (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8 .. (), (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9 .. (), (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10 .. (), (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10, T12: 11 .. (), (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10, T12: 11, T13: 12 .. (), (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10, T12: 11, T13: 12, T14: 13 .. (), ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10, T12: 11, T13: 12, T14: 13, T15: 14 .. ());
tuple_surprise!(T1: 0, T2: 1, T3: 2, T4: 3, T5: 4, T6: 5, T7: 6, T8: 7, T9: 8, T10: 9, T11: 10, T12: 11, T13: 12, T14: 13, T15: 14, T16: 15);
