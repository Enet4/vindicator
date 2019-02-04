pub extern crate noisy_float;
use std::io::BufWriter;
use std::fs::{File, read_to_string};
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
        fuser: String,
        /// Output file (print to stdout by default)
        #[structopt(parse(from_os_str), short = "o")]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::from_args();

    match app {
        App::Merge { files, fuser, output, .. } => {
            let files_data = files
                .iter()
                .map(read_to_string)
                .collect::<Result<Vec<_>, _>>()?;
            let entries = files_data
                .iter()
                .map(|data| trec::parse_from_trec(data))
                .collect::<Result<Vec<_>, _>>()?;
            let list: Vec<_> = entries.into_iter().flatten().collect();
            if let Some(list) = match &*fuser {
                "combMAX" | "combmax" | "max" => {
                    Some(fuser::fuse_scored(list, fuser::comb_max))
                },
                "combSUM" | "combsum" | "sum" => {
                    Some(fuser::fuse_scored(list, fuser::comb_sum))
                },
                "combMNZ" | "combmnz" | "mnz" => {
                    Some(fuser::fuse_scored(list, fuser::comb_mnz))
                },
                _ => {
                    eprintln!("Unknown fusion algorithm `{}`", fuser);
                    std::process::exit(-2);
                },
            } {
                // transform results into new list
                let qid = "fusion";
                let runid = "0";
                let list = list.into_iter()
                    .enumerate()
                    .map(|(i, e)| trec::TrecEntry {
                        qid: qid,
                        docno: *e.id(),
                        rank: i as Rank,
                        score: e.score(),
                        runid: runid,
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
