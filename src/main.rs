pub extern crate noisy_float;
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::path::PathBuf;
use structopt::StructOpt;

use vindicator::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "vindicator", about = "Search result list processing tool.")]
pub enum App {
    #[structopt(name = "merge", about = "Perform late fusion of search result lists")]
    Merge {
        /// The input lists
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
        /// Result fusion algorithm
        #[structopt(short = "f")]
        fuser: Fuser,
        /// The output's query name
        #[structopt(short = "q", long = "qid", default_value = "fusion")]
        qid: String,
        /// The output's run name
        #[structopt(long = "runid", default_value = "vindicated")]
        runid: String,
        /// Output file (print to stdout by default)
        #[structopt(parse(from_os_str), short = "o")]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, StructOpt)]
pub enum Fuser {
    #[structopt(name = "combMAX", alias = "combmax")]
    CombMax,
    #[structopt(name = "combSUM", alias = "combsum")]
    CombSum,
    #[structopt(name = "combMNZ", alias = "combmnz")]
    CombMnz,
}

impl std::str::FromStr for Fuser {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combMAX" | "combmax" | "max" => Ok(Fuser::CombMax),
            "combSUM" | "combsum" | "sum" => Ok(Fuser::CombMax),
            "combMNZ" | "combmnz" | "mnz" => Ok(Fuser::CombMax),
            _ => Err(format!("Unknown fusion algorithm `{}`", s)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::from_args();

    match app {
        App::Merge {
            files,
            fuser,
            output,
            qid,
            runid,
        } => {
            let files_data = files
                .iter()
                .map(read_to_string)
                .collect::<Result<Vec<_>, _>>()?;
            let entries = files_data
                .iter()
                .map(|data| trec::parse_from_trec(data))
                .collect::<Result<Vec<_>, _>>()?;
            let list: Vec<_> = entries.into_iter().flatten().collect();
            if let Some(list) = match fuser {
                Fuser::CombMax => Some(fuser::fuse_scored(list, fuser::comb_max)),
                Fuser::CombSum => Some(fuser::fuse_scored(list, fuser::comb_sum)),
                Fuser::CombMnz => Some(fuser::fuse_scored(list, fuser::comb_mnz)),
            } {
                // transform results into new list
                let list = list.into_iter().enumerate().map(|(i, e)| trec::TrecEntry {
                    qid: &qid,
                    docno: *e.id(),
                    rank: i as Rank,
                    score: e.score(),
                    runid: &runid,
                });

                // create output stream
                match output {
                    Some(o) => {
                        let file = BufWriter::new(File::create(o)?);
                        trec::write_all(file, list)?;
                    }
                    None => {
                        trec::write_all(std::io::stdout(), list)?;
                    }
                }
            }
        }
    }

    Ok(())
}
