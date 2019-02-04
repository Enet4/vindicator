//! TREC File parsing and printing module
use crate::{Rank, RankedSearchEntry, Score, SearchEntry};
use std::fmt;
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
pub struct TrecEntry<'a> {
    /// Query ID
    pub qid: &'a str,
    /// Document number (unique identifier for a document)
    pub docno: &'a str,
    /// Rank (position of the document in the list)
    pub rank: Rank,
    /// Similarity score (higher is more similar)
    pub score: Score,
    /// Unique run ID. This is currently ignored by the fusion algorithms.
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
pub enum ParseError {
    /// Unexpected end of line before reading a specific attribute
    Eol(&'static str),
    /// Invalid rank value (must be a non-negative integer)
    InvalidRank(String),
    /// Invalid score value (must be a non-NaN number)
    InvalidScore(String),
    /// Something else happened
    Other(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;
        match self {
            Eol(att) => write!(f, "failed to parse TREC data: unexpected end of line ({})", att),
            InvalidRank(rank) => write!(f, "failed to parse TREC data: invalid rank `{}`", rank),
            InvalidScore(score) => write!(f, "failed to parse TREC data: invalid score `{}`", score),
            Other(s) => write!(f, "failed to parse TREC data: {}", s),
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        use ParseError::*;
        match *self {
            Eol(_) => "unexpected end of line",
            InvalidRank(_) => "invalid rank",
            InvalidScore(_) => "invalid score",
            Other(ref s) => s,
        }
    }
}

/// Expected format:
///
/// `qid 0 docno rank score run_id`
pub fn parse_from_trec<'a>(file_data: &'a str) -> Result<Vec<TrecEntry<'a>>, ParseError> {
    file_data
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let qid = words
                .next()
                .ok_or_else(|| ParseError::Eol("qid"))?;
            let _0 = words
                .next()
                .ok_or_else(|| ParseError::Eol("reserved"))?;
            let docno = words
                .next()
                .ok_or_else(|| ParseError::Eol("docno"))?;
            let rank = words
                .next()
                .ok_or_else(|| ParseError::Eol("rank"))?;
            let rank: u32 = rank
                .parse()
                .map_err(|_| ParseError::InvalidRank(rank.to_string()))?;
            let score = words
                .next()
                .ok_or_else(|| ParseError::Eol("score"))?;
            let score: Score = score
                .parse()
                .map_err(|_| ())
                .and_then(|s| Score::try_new(s).ok_or(()))
                .map_err(|_| ParseError::InvalidScore(score.to_string()))?;
            let runid = words
                .next()
                .ok_or_else(|| ParseError::Eol("runid"))?;
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


/// Write a single text line of this TREC result entry.
/// 
/// Format: `qid 0 docno rank score run_id` (separated by spaces)
pub fn write<W>(mut writer: W, entry: TrecEntry) -> Result<(), std::io::Error>
where
    W: Write,
{
    writeln!(writer, "{} 0 {} {} {} {}", entry.qid, entry.docno, entry.rank, entry.score, entry.runid)
}

/// Write a list of TREC result entries.
/// 
/// Format: `qid 0 docno rank score run_id` (separated by spaces)
pub fn write_all<'a, I, W>(mut writer: W, list: I) -> Result<(), std::io::Error>
where
    I: IntoIterator<Item = TrecEntry<'a>>,
    W: Write,
{
    for e in list {
        write(&mut writer, e)?;
    }
    Ok(())
}
