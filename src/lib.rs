//! Search manipulation algorithms for multi-source information retrieval.
use noisy_float::prelude::*;
use approx::AbsDiffEq;

pub mod fuser;
pub mod trec;

pub type Score = N32;
pub type Rank = u32;

/// Create a score value.
/// 
/// # Panic
/// 
/// Panics if the given value is `NaN`.
pub fn score(value: f32) -> Score {
    n32(value)
}

pub trait SearchEntry {
    type Id: Eq;
    fn id(&self) -> &Self::Id;
    fn score(&self) -> Score {
        n32(1.)
    }
    fn to_entry(&self) -> EntryInfo<Self::Id>
    where
        Self::Id: Clone,
    {
        EntryInfo {
            id: self.id().clone(),
            score: self.score(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EntryInfo<I> {
    pub id: I,
    pub score: Score,
}

impl<I> AbsDiffEq for EntryInfo<I>
where
    I: PartialEq<I>,
{
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        1e-5
    }
    
    fn abs_diff_eq(
        &self, 
        other: &Self, 
        epsilon: Self::Epsilon, 
    ) -> bool {
        self.id == other.id
            && self.score.raw().abs_diff_eq(&other.score.raw(), epsilon)
    }
}

/// Wrapper type for assigning a rank to an arbitrary value.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ranked<T> {
    pub inner: T,
    pub rank: Rank,
}

impl<T> SearchEntry for Ranked<T>
where
    T: SearchEntry,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        self.inner.id()
    }

    fn score(&self) -> Score {
        self.inner.score()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RankedEntryInfo<I> {
    pub id: I,
    pub score: Score,
    pub rank: Rank,
}

impl<'a, T: ?Sized> SearchEntry for &'a T
where
    T: SearchEntry,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        (**self).id()
    }
    fn score(&self) -> Score {
        (**self).score()
    }
    fn to_entry(&self) -> EntryInfo<Self::Id>
    where
        Self::Id: Clone,
    {
        (**self).to_entry()
    }
}

pub trait RankedSearchEntry: SearchEntry {
    fn rank(&self) -> Rank;
}

impl<'a, T: ?Sized> RankedSearchEntry for &'a T
where
    T: RankedSearchEntry,
{
    fn rank(&self) -> Rank {
        (**self).rank()
    }
}

impl<I> SearchEntry for EntryInfo<I>
where
    I: Eq,
{
    type Id = I;

    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn score(&self) -> Score {
        self.score
    }

    fn to_entry(&self) -> EntryInfo<I>
    where
        I: Clone,
    {
        self.clone()
    }
}

impl<I> SearchEntry for RankedEntryInfo<I>
where
    I: Eq,
{
    type Id = I;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn score(&self) -> Score {
        self.score
    }
}

impl<I> RankedSearchEntry for RankedEntryInfo<I>
where
    I: Eq,
{
    fn rank(&self) -> Rank {
        self.rank
    }
}

pub fn ranked_list<L, R>(results: L) -> impl Iterator<Item = Ranked<R>>
where
    L: IntoIterator<Item = R>,
    R: SearchEntry,
{
    results.into_iter().enumerate().map(|(i, x)| Ranked {
        inner: x,
        rank: i as Rank,
    })
}
