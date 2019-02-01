//! File parsing module
use vindicator::{Rank, RankedSearchEntry, Score, SearchEntry};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct TrecEntry<'a> {
    pub qid: &'a str,
    pub docno: &'a str,
    pub rank: Rank,
    pub score: Score,
    pub runid: &'a str,
}

impl<'a> TrecEntry<'a> {
    pub fn to_owned(&self) -> TrecEntryOwned {
        TrecEntryOwned {
            qid: self.qid.to_string(),
            docno: self.docno.to_string(),
            rank: self.rank,
            score: self.score,
            runid: self.runid.to_string(),
        }
    }
}

impl<'a> SearchEntry for TrecEntry<'a> {
    type Id = &'a str;
    fn id(&self) -> &Self::Id {
        &self.docno
    }

    fn score(&self) -> Score {
        self.score
    }
}

impl<'a> RankedSearchEntry for TrecEntry<'a> {
    fn rank(&self) -> Rank {
        self.rank
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TrecEntryOwned {
    pub qid: String,
    pub docno: String,
    pub rank: Rank,
    pub score: Score,
    pub runid: String,
}

impl SearchEntry for TrecEntryOwned {
    type Id = String;
    fn id(&self) -> &Self::Id {
        &self.docno
    }

    fn score(&self) -> Score {
        self.score
    }
}

impl RankedSearchEntry for TrecEntryOwned {
    fn rank(&self) -> Rank {
        self.rank
    }
}

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

/// Expected format:
///
/// `qid 0 docno rank sim run_id`
pub fn parse_from_trec<'a>(file_data: &'a str) -> Result<Vec<TrecEntry<'a>>, ParseError> {
    file_data
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let qid = words
                .next()
                .ok_or_else(|| ParseError("unexpected end of line (qid)".to_string()))?;
            let docno = words
                .next()
                .ok_or_else(|| ParseError("unexpected end of line (docno)".to_string()))?;
            let rank = words
                .next()
                .ok_or_else(|| ParseError("unexpected end of line (rank)".to_string()))?;
            let rank: u32 = rank
                .parse()
                .map_err(|_| ParseError(format!("invalid rank number `{}`", rank)))?;
            let score = words
                .next()
                .ok_or_else(|| ParseError("unexpected end of line (score)".to_string()))?;
            let score: f32 = score
                .parse()
                .map_err(|_| ParseError(format!("invalid rank number `{}`", rank)))?;
            let score = Score::try_new(score)
                .ok_or_else(|| ParseError("invalid score value (must not be NaN)".to_string()))?;
            let runid = words
                .next()
                .ok_or_else(|| ParseError("unexpected end of line (runid)".to_string()))?;
            Ok(TrecEntry {
                qid,
                docno,
                rank,
                score,
                runid,
            })
        })
        .collect()
}
