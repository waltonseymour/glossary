extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
mod search;
mod write;

fn main() {
    let matches = App::new("glossary")
        .version("0.1.0")
        .about("Fast and lightweight flat file indexer")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("index")
                .arg(Arg::with_name("file").required(true))
                .arg(Arg::with_name("key_index").default_value("0")),
        )
        .subcommand(
            SubCommand::with_name("find")
                .arg(Arg::with_name("file").required(true))
                .arg(Arg::with_name("key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("index", Some(sub_m)) => {
            let filename = sub_m.value_of("file").expect("no filename passed");
            let mut f = std::fs::File::open(filename).expect("could not open file");
            let key_index = sub_m
                .value_of("key_index")
                .expect("no key_index passed")
                .parse::<usize>()
                .expect("could not parse key_index");

            write::generate_index(&mut f, key_index);
        }
        ("find", Some(sub_m)) => {
            let filename = sub_m.value_of("file").expect("no filename passed");
            let mut f = std::fs::File::open(filename).expect("could not open file");
            let key = sub_m.value_of("key").expect("no key passed");

            let entry = search::get_matching_row(&mut f, String::from(key));
            println!("{:?}", entry);
        }
        _ => {}
    }
}
