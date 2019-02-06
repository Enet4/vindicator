//! Late fusion algorithms.

use crate::{EntryInfo, Rank, RankedSearchEntry, Score, SearchEntry, score};
use noisy_float::prelude::*;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use std::hash::Hash;

/// CombMAX algorithm
///
/// Returns the highest score.
pub fn comb_max(scores: &[Score]) -> Score {
    scores.into_iter().cloned().max().unwrap_or(n32(0.))
}

/// CombSUM algorithm
///
/// Returns the sum of all scores.
pub fn comb_sum(scores: &[Score]) -> Score {
    scores.into_iter().cloned().sum::<Score>()
}

/// CombMNZ algorithm
///
/// Returns the sum of all scores, multiplied by the number of scores.
pub fn comb_mnz(scores: &[Score]) -> Score {
    n32(scores.len() as f32) * comb_sum(scores)
}

/// Reciprocal rank fusion algorithm
pub fn rrf(ranks: &[Rank]) -> Score {
    ranks.into_iter().map(|&r| 1. / (1. + r as f32)).map(score).sum()
}

/// Combines two lists of scored results with a score-based fusion algorithm.
/// Since it's score based, this is equivalent to chaining the
/// two lists together and calling [`fuse_scored`].
///
/// [`fuse_scored`]: ./fn.fuse_scored.html
pub fn fuse_scored_lists<I, L1, L2, R1, R2, F>(
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

    fuse_scored(Iterator::chain(results1, results2), fuser)
}

/// Combines multiple scored results with a score-based fusion algorithm.
pub fn fuse_scored<I, L, R, F>(results: L, fuser: F) -> Vec<EntryInfo<I>>
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

/// Combines multiple ranked results with a rank-based fusion algorithm.
pub fn fuse_ranked<I, L, R, F>(results: L, fuser: F) -> Vec<EntryInfo<I>>
where
    I: Eq + Clone + Hash,
    L: IntoIterator<Item = R>,
    R: RankedSearchEntry<Id = I>,
    F: Fn(&[Rank]) -> Score,
{
    let mut map: HashMap<I, SmallVec<[_; 4]>> = HashMap::new();

    for r in results {
        if let Some(v) = map.get_mut(r.id()) {
            v.push(r.rank());
        } else {
            map.insert(r.id().clone(), smallvec![r.rank()]);
        }
    }

    let mut flat: Vec<_> = map
        .into_iter()
        .map(|(id, ranks)| {
            // score fusion happens here
            let score = fuser(&ranks);
            EntryInfo { id, score }
        })
        .collect();

    flat.sort_unstable_by_key(|e| -e.score);
    flat
}

/// Combines multiple ranked results with a fusion algorithm based on both rank
/// and score.
pub fn fuse_hybrid<I, L, R, F>(results: L, fuser: F) -> Vec<EntryInfo<I>>
where
    I: Eq + Clone + Hash,
    L: IntoIterator<Item = R>,
    R: RankedSearchEntry<Id = I>,
    F: Fn(&[(Rank, Score)]) -> Score,
{
    let mut map: HashMap<I, SmallVec<[_; 4]>> = HashMap::new();

    for r in results {
        if let Some(v) = map.get_mut(r.id()) {
            v.push((r.rank(), r.score()));
        } else {
            map.insert(r.id().clone(), smallvec![(r.rank(), r.score())]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::score;

    #[test]
    fn test_comb_max() {
        assert_eq!(
            comb_max(&[score(1.), score(40.), score(0.5), score(12.)]),
            40.
        )
    }

    #[test]
    fn test_comb_sum() {
        assert_eq!(
            comb_sum(&[score(1.), score(40.), score(0.5), score(12.)]),
            53.5
        )
    }

    #[test]
    fn test_comb_mnz() {
        assert_eq!(
            comb_mnz(&[score(1.), score(40.), score(0.5), score(12.)]),
            214.
        )
    }
}
