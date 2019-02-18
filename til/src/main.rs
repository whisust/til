extern crate clap;
extern crate chrono;

use std::env;
use clap::{Arg, App};
use std::fs::OpenOptions;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::error::Error;

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
                .default_value("til.tsv")
                .takes_value(true)
        ).get_matches();

    let learned: &str = matches.value_of("learned").unwrap();
    let tags: Vec<&str> = matches.value_of("tags").unwrap().split(",").collect();
    let file: &str = matches.value_of("file").expect("SHould not be empty");

    println!("{}, {:?}, witten in {}", learned, tags, file);
    add(learned, tags, file);
}

fn add(learned:&str, tags: Vec<&str>, filename: &str) -> u8 {
    let mut options = OpenOptions::new();
    options.append(true);
    options.create(true);
    let mut file: File = options.open(filename).expect("Unable to open file");
    let now: DateTime<Utc> = Utc::now();
    let r = file.write_all(
        format!("{}\t{}\t{}\n", now.to_rfc3339(), learned, tags.join(","))
            .as_bytes()
    );
    match r {
        Err(_) => { 1 }
        Ok(_) => { 0 }
    }
}
