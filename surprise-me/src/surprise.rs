use rand::{prelude::Distribution, Rng};

/// Trait to generate random instances of types.
///
/// # Derive macro
///
/// The macro has two jobs:
/// - Create a [`SurpriseFactor`] (unless one is already specified, see recursive types)
/// - Implement [`Surprise`] with that surprise factor
///
/// Types themselves can be annotated with `#[factor = "MySurpriseFactor"]` in which
/// case the macro won't create a new type and instead use the specified name as [`SurpriseFactor`].
///
/// ## On structs
///
/// Fields can be annotated with:
/// - `#[factor(field = ...)]` or `#[factor(field(...))]`: These attributes modify the
/// default values for the surprise factor. `field` can be the name of any field
/// of the type's [`SurpriseFactor`]. Note that this works recursively for inner fields.
/// - `#[factor = "MySurpriseFactor"]`: Same as for the outer struct; specifying a factor
/// like this will prevent the macro from creating a new type.
///
/// ```rust
/// # use surprise_me::Surprise;
/// #
/// #[derive(Surprise)]
/// pub(crate) struct MyStruct {
///     // at most 5 elements
///     #[factor(max_len = 5)]
///     // 30% chance of being `Some` containing a value between 10 and 20
///     #[factor(items(chance = 0.3, inner(min = 10, max = 20)))]
///     my_field: Vec<Option<u32>>,
/// }
///
/// #[derive(Surprise)]
/// struct MyWrapper(
///     // same defaults except the minimum length is now 2
///     #[factor(my_field(min_len = 2))]
///     MyStruct
/// );
/// ```
///
/// In order to use attributes recursively, it helps to understand what the
/// generated surprise factor looks like:
///
/// ```rust
/// # use surprise_me::factors::{NumberSurprise, OptionSurprise, VecSurprise};
/// #
/// pub(crate) struct MyStructSurprise {
///     pub my_field: VecSurprise<Option<u32>>
/// }
///
/// impl Default for MyStructSurprise {
///     fn default() -> Self {
///         Self {
///             my_field: VecSurprise {
///                 max_len: 5,
///                 items: OptionSurprise {
///                     chance: 0.3,
///                     inner: NumberSurprise {
///                         min: 10,
///                         max: 20,
///                     },
///                 },
///                 min_len: Default::default(),
///             }
///         }
///     }
/// }
///
/// struct MyWrapperSurprise(pub MyStructSurprise);
///
/// impl Default for MyWrapperSurprise {
///     fn default() -> Self {
///         let mut inner = MyStructSurprise::default();
///         inner.my_field.min_len = 2;
///
///         Self(inner)
///     }
/// }
/// ```
///
/// ## On enums
///
/// Variants can be annotated with `#[weight = number]` to specify how
/// likely it is for one variant to be generated compared to other variants.
/// For example, a variant with weight 2 is 50% less likely to appear than
/// one with weight 3.
///
/// If no variant has a specified weight, all variants have a weight of 1.
/// If at least one variant has a specified weight, all other variants that
/// don't have a specified weight will have a weight of 0.
///
/// Fields of variants have the same attributes as fields for structs, i.e.
/// - `#[factor(field = ...)]` or `#[factor(field(...))]`
/// - `#[factor = "MySurpriseFactor")]`
///
/// ```rust
/// # use surprise_me::Surprise;
/// #
/// #[derive(Surprise)]
/// enum MyEnum {
///     #[weight = 5]
///     A(#[factor(max_len = 3)] Vec<u8>),
///     #[weight = 1.5]
///     B,
///     // no specified weight means a weight of 0 in this case
///     C {
///         #[factor(chance = 0.9)]
///         my_bool: bool,
///         another_field: i32,
///     }
/// }
/// ```
///
/// The surprise factor for this enums will look as follows:
///
/// ```rust
/// # use surprise_me::factors::{BoolSurprise, NumberSurprise, VecSurprise};
/// #
/// struct MyEnumSurprise {
///     variants_weight: Box<[f64; 3]>,
///     A: (VecSurprise<u8>,),
///     // no field `B` because it's a unit variant
///     C: (BoolSurprise, NumberSurprise<i32>),
/// }
///
/// impl Default for MyEnumSurprise {
///     fn default() -> Self {
///         Self {
///             variants_weight: Box::new([5.0, 1.5, 0.0]),
///             A: (
///                 VecSurprise {
///                     max_len: 3,
///                     ..Default::default()
///                 },
///             ),
///             C: (
///                 BoolSurprise { chance: 0.9 },
///                 NumberSurprise::default(),
///             ),
///         }
///     }
/// }
/// ```
///
/// # Recursive types
///
/// In case of recursive types, a custom surprise factor is required.
///
/// ```rust
/// use surprise_me::{Distribution, Rng, Surprise};
/// use surprise_me::factors::{OptionSurprise, VecSurprise};
///
/// #[derive(Surprise)]
/// // Instead of creating a new surprise factor, use the specified one.
/// #[factor = "RecursedSurprise"]
/// struct MyStruct {
///     my_vec: Vec<MyStruct>,
///     my_opt: Option<Box<MyStruct>>,
/// }
///
/// struct RecursedSurprise {
///     depth: usize,
///     min_len: usize,
/// }
///
/// // Surprise factors are required to implement `surprise_me::Distribution`
/// impl Distribution<MyStruct> for RecursedSurprise {
///     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyStruct {
///         // Once a certain depth is reached, don't nest any further.
///         if self.depth == 5 {
///             MyStruct {
///                 my_vec: Vec::new(),
///                 my_opt: None,
///             }
///         } else {
///             // Otherwise create the inner surprise factors.
///             // And be sure to update the depth!
///             let vec_factor = VecSurprise {
///                 min_len: self.min_len,
///                 max_len: 8, // can also be hardcoded here instead of a field
///                 items: RecursedSurprise { depth: self.depth + 1, min_len: self.min_len },
///             };
///
///             let mut opt_factor = OptionSurprise::<Box<MyStruct>>::default();
///             opt_factor.chance = 0.42;
///             // `OptionSurprise::inner` -> `BoxSurprise::inner` -> `RecursedSurprise::depth`
///             opt_factor.inner.inner.depth = self.depth + 1;
///
///             MyStruct {
///                 my_vec: Surprise::generate_with_factor(rng, &vec_factor),
///                 my_opt: Surprise::generate_with_factor(rng, &opt_factor),
///             }
///         }
///     }
/// }
///
/// impl Default for RecursedSurprise {
///     fn default() -> Self {
///         Self {
///             depth: 0,
///             min_len: 3,
///         }
///     }
/// }
/// ```
pub trait Surprise: Sized {
    /// Surprise factor that guides the random generation.
    type Factor: Distribution<Self>;

    /// Generate a random instance of the type by using a default surprise factor.
    #[inline]
    fn generate<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
        SurpriseFactor<Self>: Default,
    {
        let factor = SurpriseFactor::<Self>::default();

        <Self as Surprise>::generate_with_factor(rng, &factor)
    }

    /// Generate a random instance of the type by using the given surprise factor.
    #[inline]
    fn generate_with_factor<R, F>(rng: &mut R, factor: &F) -> Self
    where
        R: Rng + ?Sized,
        F: Distribution<Self>,
    {
        rng.sample(factor)
    }
}

/// The surprise factor on which the random generation of `T` depends on.
///
/// For example, the surprise factor [`VecSurprise`] dictates
/// the minimum and maximum length of a [`Vec`], as well as the surprise factor for the
/// items themselves.
///
/// [`VecSurprise`]: crate::factors::VecSurprise
pub type SurpriseFactor<T> = <T as Surprise>::Factor;
