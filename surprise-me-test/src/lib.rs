#[cfg(test)]
mod tests {
    use std::{collections::HashMap, marker::PhantomData, num::NonZeroI32};

    use surprise_me::{
        factors::NumberSurprise,
        rand::{thread_rng, Rng},
        Distribution, Surprise,
    };

    #[test]
    fn unit_struct() {
        #[derive(Surprise)]
        struct Unit;

        let _ = Unit::generate(&mut thread_rng());
    }

    #[test]
    fn tuple_struct() {
        #[derive(Surprise)]
        struct Tuple(u32, bool, #[factor(max_len = 10)] String);

        let _ = Tuple::generate(&mut thread_rng());
    }

    #[test]
    fn named_struct() {
        #[derive(Surprise)]
        #[allow(unused)]
        struct Named {
            #[factor(chance = 0.2)]
            a: bool,
            b: i32,
            #[factor(max_len = 10)]
            longer_name: Vec<u8>,
        }

        let _ = Named::generate(&mut thread_rng());
    }

    #[test]
    fn my_enum() {
        use surprise_me::factors::CharSurprise;

        #[derive(Surprise)]
        #[allow(unused)]
        enum MyEnum {
            A,
            #[weight = 0.1]
            B(i8),
            #[weight = 0.4]
            C(
                u32,
                #[factor(max_len = 15, chars(CharSurprise::Unicode))] String,
            ),
            D {
                field: (),
            },
            #[weight = 1]
            /// Variant documentation
            E {
                #[factor(chance = 0.9)]
                a: bool,
                #[factor(min_len = 2, max_len = 10)]
                b: Vec<u8>,
                c: HashMap<u8, Vec<f32>>,
            },
            F {},
            G(),
        }

        let _ = MyEnum::generate(&mut thread_rng());
    }

    #[test]
    #[should_panic]
    fn zero_non_zero_int() {
        let factor = NumberSurprise::<i32> { min: 0, max: 0 };

        let _ = NonZeroI32::generate_with_factor(&mut thread_rng(), &factor);
    }

    #[test]
    fn recursive_tuple_struct() {
        #[derive(Surprise)]
        #[allow(unused)]
        struct Recursive(bool, #[factor = "RecursedSurprise"] Option<Box<Recursive>>);

        #[derive(Clone)]
        #[allow(unused)]
        struct RecursedSurprise {
            depth: usize,
            chance: f64,
        }

        impl Distribution<Option<Box<Recursive>>> for RecursedSurprise {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<Box<Recursive>> {
                if self.depth < 5 && rng.gen_bool(self.chance) {
                    let mut factor = RecursiveSurprise::default();
                    factor.1.depth = self.depth + 1;

                    Some(Box::new(Recursive::generate_with_factor(rng, &factor)))
                } else {
                    None
                }
            }
        }

        impl Default for RecursedSurprise {
            fn default() -> Self {
                Self {
                    depth: 0,
                    chance: 0.3,
                }
            }
        }

        let _ = Recursive::generate(&mut thread_rng());
    }

    #[test]
    fn recursive_named_struct() {
        #[derive(Surprise)]
        #[allow(unused)]
        struct Recursive {
            other: bool,
            #[factor = "RecursedSurprise"]
            inner: Option<Box<Recursive>>,
        }

        #[derive(Clone)]
        #[allow(unused)]
        struct RecursedSurprise {
            depth: usize,
            chance: f64,
        }

        impl Distribution<Option<Box<Recursive>>> for RecursedSurprise {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<Box<Recursive>> {
                if self.depth < 5 && rng.gen_bool(self.chance) {
                    let mut factor = RecursiveSurprise::default();
                    factor.inner.depth = self.depth + 1;

                    Some(Box::new(Recursive::generate_with_factor(rng, &factor)))
                } else {
                    None
                }
            }
        }

        impl Default for RecursedSurprise {
            fn default() -> Self {
                Self {
                    depth: 0,
                    chance: 0.3,
                }
            }
        }

        let _ = Recursive::generate(&mut thread_rng());
    }

    #[test]
    fn recursive_enum() {
        #[derive(Surprise)]
        #[allow(unused)]
        enum RecursiveEnum {
            #[weight = 2.0]
            A(Option<Box<Intermediate>>),
            B,
            #[weight = 1.0]
            C {
                #[factor = "RecursedEnum"]
                direct: Box<RecursiveEnum>,
            },
            #[weight = 5.0]
            D(#[factor = "RecursedEnum"] Vec<RecursiveEnum>),
        }

        #[derive(Surprise)]
        #[allow(unused)]
        struct Intermediate {
            #[factor = "RecursedEnum"]
            inner: RecursiveEnum,
        }

        struct RecursedEnum {
            depth: usize,
            min_len: usize,
            max_len: usize,
        }

        impl Distribution<RecursiveEnum> for RecursedEnum {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RecursiveEnum {
                let mut factor = RecursiveEnumSurprise::default();

                factor.A.0.inner.inner.inner.depth = self.depth + 1;
                factor.C.0.depth = self.depth + 1;
                factor.D.0.depth = self.depth + 1;

                RecursiveEnum::generate_with_factor(rng, &factor)
            }
        }

        impl Distribution<Box<RecursiveEnum>> for RecursedEnum {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<RecursiveEnum> {
                let mut factor = RecursiveEnumSurprise::default();

                factor.A.0.inner.inner.inner.depth = self.depth + 1;
                factor.C.0.depth = self.depth + 1;
                factor.D.0.depth = self.depth + 1;

                Box::new(RecursiveEnum::generate_with_factor(rng, &factor))
            }
        }

        impl Distribution<Vec<RecursiveEnum>> for RecursedEnum {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<RecursiveEnum> {
                if self.depth >= 5 {
                    return Vec::new();
                }

                let mut factor = RecursiveEnumSurprise::default();

                factor.A.0.inner.inner.inner.depth = self.depth + 1;
                factor.C.0.depth = self.depth + 1;
                factor.D.0.depth = self.depth + 1;

                let len = rng.gen_range(self.min_len..=self.max_len);

                (0..len)
                    .map(|_| RecursiveEnum::generate_with_factor(rng, &factor))
                    .collect()
            }
        }

        impl Default for RecursedEnum {
            fn default() -> Self {
                Self {
                    depth: 0,
                    min_len: 0,
                    max_len: 5,
                }
            }
        }

        let _ = RecursiveEnum::generate(&mut thread_rng());
    }

    #[test]
    fn recursive_custom() {
        #[derive(Surprise)]
        #[allow(unused)]
        struct Message {
            other: bool,
            #[factor = "RecursedMessageSurprise"]
            referenced_message: Option<Box<Message>>,
        }

        #[derive(Clone)]
        struct RecursedMessageSurprise {
            is_reference: bool,
            chance: f64,
        }

        impl Distribution<Option<Box<Message>>> for RecursedMessageSurprise {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<Box<Message>> {
                if !self.is_reference && rng.gen_bool(self.chance) {
                    let factor = MessageSurprise::default();

                    Some(Box::new(Message::generate_with_factor(rng, &factor)))
                } else {
                    None
                }
            }
        }

        impl Default for RecursedMessageSurprise {
            fn default() -> Self {
                Self {
                    is_reference: false,
                    chance: 0.5,
                }
            }
        }

        let _ = Message::generate(&mut thread_rng());
    }

    #[test]
    fn generic_struct() {
        #[derive(Surprise)]
        #[allow(unused)]
        struct GenericStruct<T: Surprise, U: Surprise> {
            vec: Vec<T>,
            opt: Option<U>,
        }

        let _ = GenericStruct::<u8, bool>::generate(&mut thread_rng());
    }

    #[test]
    fn generic_enum() {
        #[derive(Surprise)]
        #[allow(unused)]
        enum GenericEnum<T: Surprise, U: Surprise, V> {
            A(Vec<T>),
            B { a: Option<U>, b: PhantomWrapper<V> },
        }

        #[derive(Surprise)]
        struct PhantomWrapper<T> {
            inner: PhantomData<T>,
        }

        let _ = GenericEnum::<u8, bool, i8>::generate(&mut thread_rng());
    }

    // Currently won't compile since defaults for nested enums cannot be set
    // #[test]
    // fn nested() {
    //     #[derive(Surprise)]
    //     #[allow(unused)]
    //     struct Base {
    //         #[factor(chance = 0.2, inner(A(0(chance = 0.9))))]
    //         field: Option<Inner>,
    //     }

    //     #[derive(Surprise)]
    //     enum Inner {
    //         A(bool, HashMap<u8, bool>),
    //         B { field: Option<Vec<u8>> },
    //     }

    //     let _ = Base::generate(&mut thread_rng());
    // }
}
