#[macro_use]
extern crate clap;

use std::env;
use clap::{Arg, App};

fn main() {
    let default_home = env::var_os("USERPROFILE").or(env::var_os("HOME"));

    let matches = App::new("til")
        .version("0.0.1")
        .author("Antoine Latrille <antoine.latrille@gmail.com>")
        .about("Store and tag what you learned to remember it easily")
        .arg(
            Arg::with_name("learned")
                .help("What you learned")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("tags")
                .help("Tags, separate by a comma")
                .short("t")
                .long("tags")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("file")
                .help("File where to read/write what you learned")
                .short("f")
                .long("file")
                .default_value("til.txt")
                .takes_value(true)
        ).get_matches();

    let learned: &str = matches.value_of("learned").unwrap();
    let tags: Vec<&str> = matches.value_of("tags").unwrap().split(",").collect();
    let file: &str = matches.value_of("file").expect("SHould not be empty");

    println!("{}, {:?}, witten in {}", learned, tags, file);
}
