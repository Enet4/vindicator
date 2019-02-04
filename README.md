# Vindicator
[![Latest Version](https://img.shields.io/crates/v/vindicator.svg)](https://crates.io/crates/vindicator) [![Build Status](https://travis-ci.org/Enet4/vindicator.svg?branch=master)](https://travis-ci.org/Enet4/vindicator) [![dependency status](https://deps.rs/repo/github/Enet4/vindicator/status.svg)](https://deps.rs/repo/github/Enet4/vindicator)

A list manipulation framework for multi-source information retrieval.

This is a work in progress, more features and major API changes may happen.

## Using the command line tool

```
USAGE:
    vindicator <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    merge    Perform late fusion of search result lists
```

### merge

```
USAGE:
    vindicator merge [OPTIONS] -f <fuser> [files]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f <fuser>         Result fusion algorithm
    -o <output>        Output file (print to stdout by default)

ARGS:
    <files>...    The input lists
```


## Using the API

Please see the [documentation](https://docs.rs/vindicator) for more.

```rust
use vindicator::{comb_mnz, fuse_scored, parse_from_trec};

let raw_data = std::fs::read_to_string("trec_file.txt")?;
let data = files_data
                .iter()
                .map(|data| trec::parse_from_trec(data))
                .collect::<Result<Vec<_>, _>>()?;
let fusion = fuse_scored(comb_mnz, comb_mnz);


```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
