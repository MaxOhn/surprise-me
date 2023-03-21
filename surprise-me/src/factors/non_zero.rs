use rand::{prelude::Distribution, Rng};

use crate::Surprise;

use super::NumberSurprise;

macro_rules! non_zero_surprise {
    (UINT: $( $non_zero:ident ($int:ty) $(,)? )+) => {
        $(
            impl Surprise for std::num:: $non_zero {
                type Factor = NumberSurprise<$int>;
            }

            impl Distribution<std::num::$non_zero> for NumberSurprise<$int> {
                #[inline]
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> std::num::$non_zero {
                    assert!(
                        self.max != 0,
                        concat!("cannot generate ", stringify!($non_zero), " from zero"),
                    );

                    let n = rng.gen_range(self.min.max(1)..=self.max);

                    // SAFETY: `n` is guaranteed to be greater equal one
                    unsafe { std::num::$non_zero::new_unchecked(n) }
                }
            }
        )*
    };
    (INT: $( $non_zero:ident ($int:ty) $(,)? )+) => {
        $(
            impl Surprise for std::num:: $non_zero {
                type Factor = NumberSurprise<$int>;
            }

            impl Distribution<std::num::$non_zero> for NumberSurprise<$int> {
                #[inline]
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> std::num::$non_zero {
                    assert!(
                        self.min != 0 || self.max != 0,
                        concat!("cannot generate ", stringify!($non_zero), " from zero"),
                    );

                    loop {
                        let n: $int = rng.gen_range(self.min..=self.max);

                        if n != 0 {
                            // SAFETY: `n` is guaranteed to be non-zero
                            return unsafe { std::num::$non_zero::new_unchecked(n) };
                        }
                    }
                }
            }
        )*
    };
}

non_zero_surprise!(
    UINT: NonZeroU8(u8),
    NonZeroU16(u16),
    NonZeroU32(u32),
    NonZeroU64(u64),
    NonZeroU128(u128),
    NonZeroUsize(usize),
);

non_zero_surprise!(
    INT: NonZeroI8(i8),
    NonZeroI16(i16),
    NonZeroI32(i32),
    NonZeroI64(i64),
    NonZeroI128(i128),
    NonZeroIsize(isize),
);
