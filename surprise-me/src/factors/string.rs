use rand::{
    distributions::{Alphanumeric, Standard},
    prelude::Distribution,
    Rng,
};

use crate::Surprise;

use super::CharSurprise;

#[derive(Clone, Debug, Eq, PartialEq)]
/// The surprise factor of [`String`]
pub struct StringSurprise {
    /// The minimum length of generated strings
    pub min_len: usize,
    /// The maximum length of generated strings
    pub max_len: usize,
    /// The surprise factor for generated characters
    pub chars: CharSurprise,
}

impl Surprise for String {
    type Factor = StringSurprise;
}

impl StringSurprise {
    #[allow(clippy::len_without_is_empty)]
    pub fn len<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        rng.gen_range(self.min_len..=self.max_len)
    }
}

impl Distribution<String> for StringSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let len = self.len(rng);

        match self.chars {
            CharSurprise::Ascii => {
                let bytes = rng.sample_iter(Alphanumeric).take(len).collect();

                // SAFETY: bytes are guaranteed to be valid ASCII characters
                unsafe { String::from_utf8_unchecked(bytes) }
            }
            CharSurprise::Unicode => rng.sample_iter::<char, _>(Standard).take(len).collect(),
        }
    }
}

impl Default for StringSurprise {
    #[inline]
    fn default() -> Self {
        Self {
            min_len: 0,
            max_len: 100,
            chars: CharSurprise::default(),
        }
    }
}
