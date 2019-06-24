//! cat reads files in sequence and writes their contents to standard
//! output in the same sequence.

use clap::{App, Arg};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    let matches = App::new("rust-cat")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Rust clone of the cat utility. Concatenate FILE(s) to standard output.\n\nWith no FILE, or when FILE is -, read standard input.")
        .arg(Arg::with_name("unbuffered")
             .short("u")
             .help("Ignored (present for POSIX compatibility)")
             .takes_value(false))
        .arg(Arg::with_name("FILE")
             .help("")
             .required(false)
             .default_value("-")
             .multiple(true)
        ).get_matches();

    let files: Vec<_> = matches.values_of("FILE").unwrap().collect();

    for filename in files {
        if filename == "-" {
            // Read standard input until EOF
            let mut contents = String::new();
            io::stdin()
                .read_to_string(&mut contents)
                .unwrap_or_else(|err| {
                    eprintln!("Error reading stdin: {}", err);
                    process::exit(1);
                });
            print!("{}", contents);
        } else {
            // Open the file and read its contents.
            let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                eprintln!("Error reading file `{}`: {}", filename, err);
                process::exit(1);
            });
            print!("{}", contents);
        }
    }
}
