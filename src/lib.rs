//! Search manipulation algorithms for multi-source information retrieval.
//! 
//! # Example
//! 
//! Read a file in the TREC eval result list format, and use
//! [`fuse_scored`] to merge entries with the same document ID.
//! 
//! ```no_run
//! use vindicator::{fuse_scored, parse_from_trec};
//! use vindicator::fuser::comb_mnz;
//! 
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let raw_list = std::fs::read_to_string("trec_file.txt")?;
//! let list = parse_from_trec(&raw_list)?;
//!
//! let fusion: Vec<_> = fuse_scored(&list, comb_mnz);
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//! 
//! [`comb_mnz`] is one possible score-based late fusion method. Any other
//! algorithm producing a new score based on an array of scores can be
//! integrated. See also [`comb_max`] and [`comb_sum`].
//! 
//! [`fuse_scored`]: fuser/fn.fuse_scored.html
//! [`comb_mnz`]: fuser/fn.comb_mnz.html
//! [`comb_max`]: fuser/fn.comb_max.html
//! [`comb_sum`]: fuser/fn.comb_sum.html

use noisy_float::prelude::*;
use approx::AbsDiffEq;

pub use noisy_float;
pub use approx;

pub mod fuser;
pub mod trec;

pub use fuser::fuse_scored;
pub use trec::parse_from_trec;

/// Type alias for a search result's score. This is assumed to be a s
pub type Score = N32;
/// Type alias for a search result's ran.
pub type Rank = u32;

/// Creates a score value.
/// 
/// # Panic
/// 
/// Panics if the given value is `NaN`.
pub fn score(value: f32) -> Score {
    n32(value)
}

/// A search result entry with a unique document identifier and a similarity
/// score. Types need to implement this type in order to be admitted as a
/// search result.
pub trait SearchEntry {
    /// The unique document identifier type.
    type Id: Eq;

    /// Retrieves this entry's document ID.
    fn id(&self) -> &Self::Id;

    /// Retrieves this entry's similarity score.
    fn score(&self) -> Score {
        n32(1.)
    }

    /// Constructs a minimalist entry info data structure.
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

/// A simple struct for minimally describing a scored search result.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EntryInfo<I> {
    /// The entry's document ID.
    pub id: I,
    /// The entry's similarity score.
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
    /// the attributed rank
    pub rank: Rank,
    /// the inner value
    pub inner: T,
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

/// A simple struct for minimally describing a scored and ranked search result.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RankedEntryInfo<I> {
    /// The entry's document ID.
    pub id: I,
    /// The entry's similarity score.
    pub score: Score,
    /// The entry's rank.
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

/// A search entry which is also aware of its rank on the list.
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

/// Builds a new iterator containing search results ranked on their order of
/// appearance.
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
