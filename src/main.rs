extern crate clap;

use clap::{Arg, Command};
mod search;
mod write;

fn main() {
    let matches = Command::new("glossary")
        .bin_name("glossary")
        .subcommand_required(true)
        .version("0.2.0")
        .about("Fast and lightweight flat file indexer")
        .subcommand(
            Command::new("index")
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("delimiter")
                        .default_value(",")
                        .value_parser(clap::value_parser!(char))
                        .short('d')
                        .long("delimiter"),
                )
                .arg(
                    Arg::new("key_index")
                        .value_parser(clap::value_parser!(usize))
                        .default_value("0")
                        .short('k')
                        .long("key-index"),
                ),
        )
        .subcommand(
            Command::new("find")
                .arg(Arg::new("file").required(true))
                .arg(Arg::new("key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("index", sub_m)) => {
            let filename = sub_m.get_one::<String>("file").expect("no filename passed");
            let mut f = std::fs::File::open(filename).expect("could not open file");
            let key_index = sub_m
                .get_one::<usize>("key_index")
                .expect("no key_index passed");

            let delimiter = sub_m
                .get_one::<char>("delimiter")
                .expect("no delimiter passed");

            write::generate_index(&mut f, *key_index, *delimiter);
        }
        Some(("find", sub_m)) => {
            let filename = sub_m.get_one::<String>("file").expect("no filename passed");
            let mut f = std::fs::File::open(filename).expect("could not open file");
            let key = sub_m.get_one::<String>("key").expect("no key passed");

            let entry = search::get_matching_row(&mut f, key.clone());
            println!("{:?}", entry);
        }
        _ => {}
    }
}
