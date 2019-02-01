//! Search manipulation algorithms for multi-source information retrieval.
use noisy_float::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use smallvec::{SmallVec, smallvec};

pub type Score = N32;
pub type Rank = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EntryInfo<I> {
    pub id: I,
    pub score: Score,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RankedEntryInfo<I> {
    pub id: I,
    pub score: Score,
    pub rank: Rank,
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

/// CombMAX algorithm
pub fn comb_max(scores: &[Score]) -> Score {
    scores.into_iter().cloned().max().unwrap_or(n32(0.))
}

/// CombSUM algorithm
pub fn comb_sum(scores: &[Score]) -> Score {
    scores.into_iter().cloned().sum::<Score>()
}

/// CombMNZ algorithm
pub fn comb_mnz(scores: &[Score]) -> Score {
    n32(scores.len() as f32) * comb_sum(scores)
}

/// combine multiple lists of results with scores
pub fn comb_scored_lists<I, L1, L2, R1, R2, F>(
    results1: L1,
    results2: L2,
    fuser: F,
) -> Vec<EntryInfo<I>>
where
    I: Eq + Clone + Hash,
    L1: IntoIterator<Item = R1>,
    L2: IntoIterator<Item = R2>,
    R1: SearchEntry<Id = I>,
    R2: SearchEntry<Id = I>,
    F: Fn(&[Score]) -> Score,
{
    let results1 = results1.into_iter().map(|x| x.to_entry());
    let results2 = results2.into_iter().map(|x| x.to_entry());

    comb_scored(Iterator::chain(results1, results2), fuser)
}

/// combine multiple results with scores
pub fn comb_scored<I, L, R, F>(results: L, fuser: F) -> Vec<EntryInfo<I>>
where
    I: Eq + Clone + Hash,
    L: IntoIterator<Item = R>,
    R: SearchEntry<Id = I>,
    F: Fn(&[Score]) -> Score,
{
    let mut map: HashMap<I, SmallVec<[_; 4]>> = HashMap::new();

    for r in results {
        if let Some(v) = map.get_mut(r.id()) {
            v.push(r.score());
        } else {
            map.insert(r.id().clone(), smallvec![r.score()]);
        }
    }

    let mut flat: Vec<_> = map
        .into_iter()
        .map(|(id, scores)| {
            // score fusion happens here
            let score = fuser(&scores);
            EntryInfo { id, score }
        })
        .collect();

    flat.sort_unstable_by_key(|e| -e.score);
    flat
}
