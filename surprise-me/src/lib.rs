#![doc = include_str!("../../README.md")]

pub use self::{
    rand::{distributions::Distribution, Rng},
    surprise::{Surprise, SurpriseFactor},
};

pub use rand;
pub use surprise_me_derive::Surprise;

/// Surprise factors of types in the standard library
pub mod factors;

mod surprise;

#[cfg(test)]
mod tests {
    use rand::{distributions::WeightedIndex, prelude::Distribution};

    use crate::{
        factors::{BoolSurprise, OptionSurprise, VecSurprise},
        rand::Rng,
        Surprise,
    };

    #[test]
    fn test_struct() {
        #[allow(unused)]
        struct MyStruct {
            a: u32,
            b: bool,
            c: String,
        }

        struct MyStructSurprise {
            a: <u32 as Surprise>::Factor,
            b: <bool as Surprise>::Factor,
            c: <String as Surprise>::Factor,
        }

        impl Distribution<MyStruct> for MyStructSurprise {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyStruct {
                #[allow(clippy::needless_borrow)]
                MyStruct {
                    a: rng.sample(&self.a),
                    b: rng.sample(&self.b),
                    c: rng.sample(&self.c),
                }
            }
        }

        impl Surprise for MyStruct {
            type Factor = MyStructSurprise;
        }
    }

    #[test]
    fn test_enum() {
        #[allow(unused)]
        enum MyEnum {
            A,
            B(u32),
            C { a: String, b: bool },
        }

        #[allow(non_snake_case)]
        struct MyEnumSurprise {
            probability_variants: Box<[f32; 3]>,
            B: (<u32 as Surprise>::Factor,),
            C: (<String as Surprise>::Factor, <bool as Surprise>::Factor),
        }

        impl Distribution<MyEnum> for MyEnumSurprise {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyEnum {
                let weights = self.probability_variants.as_ref();
                let weighted_idx = WeightedIndex::new(weights).unwrap();
                let idx = rng.sample(weighted_idx);

                #[allow(clippy::needless_borrow)]
                match idx {
                    0 => MyEnum::A,
                    1 => MyEnum::B(rng.sample(&self.B.0)),
                    2 => MyEnum::C {
                        a: rng.sample(&self.C.0),
                        b: rng.sample(&self.C.1),
                    },
                    _ => unreachable!(),
                }
            }
        }

        impl Surprise for MyEnum {
            type Factor = MyEnumSurprise;
        }
    }

    #[test]
    fn generic_struct() {
        #[allow(unused)]
        struct GenericStruct<T: Surprise> {
            vec: Vec<T>,
            opt: Option<T>,
        }

        #[allow(unused)]
        struct GenericStructSurprise<T: Surprise> {
            vec: VecSurprise<T>,
            opt: OptionSurprise<T>,
        }

        impl<T: Surprise> Distribution<GenericStruct<T>> for GenericStructSurprise<T> {
            fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> GenericStruct<T> {
                unimplemented!()
            }
        }

        impl<T: Surprise> Default for GenericStructSurprise<T> {
            fn default() -> Self {
                unimplemented!()
            }
        }
    }

    #[test]
    fn test_recursive() {
        #[allow(unused)]
        struct Direct {
            other: bool,
            inner: Option<Box<Direct>>,
        }

        impl Surprise for Direct {
            type Factor = DirectSurprise;
        }

        struct DirectSurprise {
            other: BoolSurprise,
            inner: RecursedDirectSurprise,
        }

        #[allow(clippy::needless_borrow)]
        impl Distribution<Direct> for DirectSurprise {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direct {
                Direct {
                    other: rng.sample(&self.other),
                    inner: rng.sample(&self.inner),
                }
            }
        }

        impl Default for DirectSurprise {
            fn default() -> Self {
                Self {
                    other: BoolSurprise { chance: 0.2 },
                    inner: Default::default(),
                }
            }
        }

        struct RecursedDirectSurprise {
            other: BoolSurprise,
            depth: usize,
            chance: f64,
        }

        impl Distribution<Option<Box<Direct>>> for RecursedDirectSurprise {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<Box<Direct>> {
                const RECURSE_LIMIT: usize = 5;

                if self.depth < RECURSE_LIMIT && rng.gen_bool(self.chance) {
                    let factor = DirectSurprise {
                        other: self.other,
                        inner: RecursedDirectSurprise {
                            other: self.other,
                            depth: self.depth + 1,
                            chance: self.chance,
                        },
                    };

                    Some(Box::new(Direct::generate_with_factor(rng, &factor)))
                } else {
                    None
                }
            }
        }

        impl Default for RecursedDirectSurprise {
            fn default() -> Self {
                Self {
                    other: BoolSurprise { chance: 0.2 },
                    depth: 0,
                    chance: 0.5,
                }
            }
        }

        let _ = Direct::generate(&mut rand::thread_rng());
    }
}
