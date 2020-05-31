extern crate clap;

use clap::{App, Arg, SubCommand};
mod search;
mod write;

fn main() {
    let matches = App::new("glossary")
        .version("0.1.0")
        .about("Fast and lightweight flat file indexer")
        .subcommand(
            SubCommand::with_name("index")
                .arg(Arg::with_name("file").required(true))
                .arg(Arg::with_name("key")),
        )
        .subcommand(SubCommand::with_name("find"))
        .get_matches();

    match matches.subcommand() {
        ("index", Some(sub_m)) => {
            let filename = sub_m.value_of("file").expect("no filename passed");
            let f = std::fs::File::open(filename).expect("could not open file");
            write::generate_index(f);
        }
        ("find", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("no key passed");

            search::find_key(String::from(key));
        }
        _ => {}
    }
}
