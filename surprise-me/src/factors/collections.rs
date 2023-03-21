use std::{
    collections::{hash_map::RandomState, BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    collections::{LinkedList, VecDeque},
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::{BuildHasher, Hash},
};

use rand::{prelude::Distribution, Rng};

use crate::{Surprise, SurpriseFactor};

/// The surprise factor of [`Vec`]
pub struct VecSurprise<T: Surprise> {
    /// The minimum length of generated vecs
    pub min_len: usize,
    /// The maximum length of generated vecs
    pub max_len: usize,
    /// The surprise factor for the items
    pub items: SurpriseFactor<T>,
}

impl<T: Surprise> Surprise for Vec<T> {
    type Factor = VecSurprise<T>;
}

impl<T: Surprise> VecSurprise<T> {
    #[allow(clippy::len_without_is_empty)]
    /// Returns a random length within the `min_len` and `max_len` values
    pub fn len<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        rng.gen_range(self.min_len..=self.max_len)
    }
}

impl<T: Surprise> Distribution<Vec<T>> for VecSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<T> {
        (0..self.len(rng)).map(|_| self.items.sample(rng)).collect()
    }
}

impl<T> Default for VecSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            min_len: 0,
            max_len: 100,
            items: Default::default(),
        }
    }
}

impl<T> Clone for VecSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            min_len: self.min_len,
            max_len: self.max_len,
            items: self.items.clone(),
        }
    }
}

impl<T> Debug for VecSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("VecSurprise")
            .field("min_len", &self.min_len)
            .field("max_len", &self.max_len)
            .field("items", &self.items)
            .finish()
    }
}

impl<T> PartialEq for VecSurprise<T>
where
    T: Surprise,
    <T as Surprise>::Factor: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.min_len == other.min_len && self.max_len == other.max_len && self.items == other.items
    }
}

/// The surprise factor of [`VecDeque`]
pub type VecDequeSurprise<T> = VecSurprise<T>;

impl<T: Surprise> Surprise for VecDeque<T> {
    type Factor = VecDequeSurprise<T>;
}

impl<T: Surprise> Distribution<VecDeque<T>> for VecDequeSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VecDeque<T> {
        (0..self.len(rng)).map(|_| self.items.sample(rng)).collect()
    }
}

/// The surprise factor of [`LinkedList`]
pub type LinkedListSurprise<T> = VecSurprise<T>;

impl<T: Surprise> Surprise for LinkedList<T> {
    type Factor = LinkedListSurprise<T>;
}

impl<T: Surprise> Distribution<LinkedList<T>> for LinkedListSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LinkedList<T> {
        (0..self.len(rng)).map(|_| self.items.sample(rng)).collect()
    }
}

use super::UnitSurprise;

/// The surprise factor of [`RandomState`]
pub type RandomStateSurprise = UnitSurprise;

impl Surprise for RandomState {
    type Factor = RandomStateSurprise;
}

impl Distribution<RandomState> for RandomStateSurprise {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> RandomState {
        RandomState::new()
    }
}

/// The surprise factor of [`HashMap`]
pub struct HashMapSurprise<K: Surprise, V: Surprise, S: Surprise = RandomState> {
    /// The minimum amount of items in the generated map
    pub min_len: usize,
    /// The maximum amount of items in the generated map
    pub max_len: usize,
    /// The surprise factor for keys
    pub keys: SurpriseFactor<K>,
    /// The surprise factor for values
    pub values: SurpriseFactor<V>,
    /// The surprise factor for the hasher
    pub hasher: SurpriseFactor<S>,
}

impl<K, V, S> Surprise for HashMap<K, V, S>
where
    K: Surprise + Eq + Hash,
    V: Surprise,
    S: Surprise + BuildHasher + Default,
{
    type Factor = HashMapSurprise<K, V, S>;
}

impl<K: Surprise, V: Surprise, S: Surprise> HashMapSurprise<K, V, S> {
    #[allow(clippy::len_without_is_empty)]
    /// Returns a random length within the `min_len` and `max_len` values
    pub fn len<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        rng.gen_range(self.min_len..=self.max_len)
    }
}

impl<K, V, S> Distribution<HashMap<K, V, S>> for HashMapSurprise<K, V, S>
where
    K: Surprise + Eq + Hash,
    V: Surprise,
    S: Surprise + BuildHasher + Default,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HashMap<K, V, S> {
        (0..self.len(rng))
            .map(|_| (self.keys.sample(rng), self.values.sample(rng)))
            .collect()
    }
}

impl<K, V, S> Clone for HashMapSurprise<K, V, S>
where
    K: Surprise,
    V: Surprise,
    S: Surprise,
    SurpriseFactor<K>: Clone,
    SurpriseFactor<V>: Clone,
    SurpriseFactor<S>: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            min_len: self.min_len,
            max_len: self.max_len,
            keys: self.keys.clone(),
            values: self.values.clone(),
            hasher: self.hasher.clone(),
        }
    }
}

impl<K, V, S> Debug for HashMapSurprise<K, V, S>
where
    K: Surprise,
    V: Surprise,
    S: Surprise,
    SurpriseFactor<K>: Debug,
    SurpriseFactor<V>: Debug,
    SurpriseFactor<S>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("HashMapSurprise")
            .field("min_len", &self.min_len)
            .field("max_len", &self.max_len)
            .field("keys", &self.keys)
            .field("values", &self.values)
            .field("hasher", &self.hasher)
            .finish()
    }
}

impl<K, V, S> Default for HashMapSurprise<K, V, S>
where
    K: Surprise,
    V: Surprise,
    S: Surprise,
    SurpriseFactor<K>: Default,
    SurpriseFactor<V>: Default,
    SurpriseFactor<S>: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            min_len: 0,
            max_len: 100,
            keys: Default::default(),
            values: Default::default(),
            hasher: Default::default(),
        }
    }
}

impl<K, V, S> PartialEq for HashMapSurprise<K, V, S>
where
    K: Surprise,
    V: Surprise,
    S: Surprise,
    SurpriseFactor<K>: PartialEq,
    SurpriseFactor<V>: PartialEq,
    SurpriseFactor<S>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.min_len == other.min_len
            && self.max_len == other.max_len
            && self.keys == other.keys
            && self.values == other.values
            && self.hasher == other.hasher
    }
}

/// The surprise factor of [`HashSet`]
pub type HashSetSurprise<T, S = RandomState> = HashMapSurprise<T, (), S>;

impl<T, S> Surprise for HashSet<T, S>
where
    T: Surprise + Eq + Hash,
    S: Surprise + BuildHasher + Default,
{
    type Factor = HashSetSurprise<T, S>;
}

impl<T, S> Distribution<HashSet<T, S>> for HashSetSurprise<T, S>
where
    T: Surprise + Eq + Hash,
    S: Surprise + BuildHasher + Default,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HashSet<T, S> {
        (0..self.len(rng)).map(|_| self.keys.sample(rng)).collect()
    }
}

/// The surprise factor of [`BTreeMap`]
pub type BTreeMapSurprise<K, V> = HashMapSurprise<K, V, ()>;

impl<K, V> Surprise for BTreeMap<K, V>
where
    K: Surprise + Ord,
    V: Surprise,
{
    type Factor = BTreeMapSurprise<K, V>;
}

impl<K, V> Distribution<BTreeMap<K, V>> for BTreeMapSurprise<K, V>
where
    K: Surprise + Ord,
    V: Surprise,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BTreeMap<K, V> {
        (0..self.len(rng))
            .map(|_| (self.keys.sample(rng), self.values.sample(rng)))
            .collect()
    }
}

/// The surprise factor of [`BTreeSet`]
pub type BTreeSetSurprise<T> = VecSurprise<T>;

impl<T: Surprise + Ord> Surprise for BTreeSet<T> {
    type Factor = BTreeSetSurprise<T>;
}

impl<T: Surprise + Ord> Distribution<BTreeSet<T>> for BTreeSetSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BTreeSet<T> {
        (0..self.len(rng)).map(|_| self.items.sample(rng)).collect()
    }
}

/// The surprise factor of [`BinaryHeap`]
pub type BinaryHeadSurprise<T> = VecSurprise<T>;

impl<T: Surprise + Ord> Surprise for BinaryHeap<T> {
    type Factor = BinaryHeadSurprise<T>;
}

impl<T: Surprise + Ord> Distribution<BinaryHeap<T>> for BinaryHeadSurprise<T> {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BinaryHeap<T> {
        (0..self.len(rng)).map(|_| self.items.sample(rng)).collect()
    }
}
