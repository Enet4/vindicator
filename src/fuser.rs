use crate::{Score, SearchEntry, EntryInfo};
use noisy_float::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use smallvec::{SmallVec, smallvec};

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

/// Combine multiple scored results with an algorithm that does not
/// depend on the entries' rank.
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


#[cfg(test)]
mod tests {
    use crate::score;
    use super::*;

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