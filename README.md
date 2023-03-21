# surprise-me

Generating random type instances.

[`Surprise`] is a simple trait whose sole purpose is to generate random things.

If you need a more sophisticated approach to prop testing and the like, check out [proptest](https://docs.rs/proptest/latest) and [quicktest](https://docs.rs/quickcheck/latest).

## Deriving the trait

```rust
use std::collections::HashMap;
use surprise_me::{Surprise, rand};

#[derive(Surprise)]
pub struct MyStruct {
    a: u32,
    // Optionally adjust the surprise factor with the `factor` attribute
    #[factor(chance = 0.4)]
    b: bool,
    #[factor(min_len = 5, max_len = 10)]
    c: HashMap<u8, f32>,
}

#[derive(Surprise)]
pub enum MyEnum {
    // Optionally specify variant weights (no weight means 0.0)
    #[weight = 0.1]
    A,
    #[weight = 0.6]
    B {
        #[factor(max_len = 25)]
        a: Vec<Option<i16>>,
        b: MyStruct,
    },
    #[weight = 0.2]
    C(u8, #[factor(min = -2.0)] f32),
}

let mut rng = rand::thread_rng();
let value: MyEnum = Surprise::generate(&mut rng);
```

## Using the trait

```rust
use surprise_me::Surprise;
use surprise_me::factors::{NumberSurprise, VecSurprise};
use surprise_me::rand; // Re-export of the rand crate

let mut rng = rand::thread_rng();

// Generate a vec filled with random bytes
let vec: Vec<u8> = Surprise::generate(&mut rng);

// Generating as above uses the type's default surprise factor.
// In the case of a vec that means a maximum length of 100.
// To fine-tune these factors, create one and pass it to the generation like so:
let factor = VecSurprise {
    min_len: 0,
    max_len: 10,
    items: NumberSurprise {
        min: b'A',
        max: b'Z',
    },
};

// Vec containing at most 10 random capital ASCII letter bytes
let vec: Vec<u8> = Surprise::generate_with_factor(&mut rng, &factor);
```