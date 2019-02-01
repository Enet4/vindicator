pub extern crate noisy_float;
use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

use vindicator::*;

mod parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "vindicator", about = "Search result list processing tool.")]
pub enum App {
    #[structopt(name = "merge", about = "Perform late fusion of search result lists")]
    Merge {
        /// The input lists
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
        fuser: String,
        /// Output file (print to stdout by default)
        #[structopt(parse(from_os_str), short = "o")]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use App::*;
    let app = App::from_args();

    match app {
        Merge { files, .. } => {
            let files_data = files
                .iter()
                .map(read_to_string)
                .collect::<Result<Vec<_>, _>>()?;
            let entries = files_data
                .iter()
                .map(|data| parse::parse_from_trec(data))
                .collect::<Result<Vec<Vec<_>>, _>>()?;
        }
    }

    Ok(())
}
