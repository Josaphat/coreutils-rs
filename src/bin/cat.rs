//! cat reads files in sequence and writes their contents to standard
//! output in the same sequence.

use clap::{App, Arg};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;
use std::str;

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

    // unwrap is fine here; FILE will have a default.
    let files: Vec<_> = matches.values_of("FILE").unwrap().collect();

    for filename in files {
        let reader: Box<dyn io::Read> = if filename == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(fs::File::open(filename).unwrap_or_else(|err| {
                eprintln!("Error reading file `{}`: {}", filename, err);
                process::exit(1);
            }))
        };
        dump_file(reader, filename)
    }
}

fn dump_file(mut reader: Box<dyn io::Read>, filename: &str) {
    const BUF_SIZE: usize = 1024;

    let mut buffer = [0; BUF_SIZE];
    loop {
        let n = reader.read(&mut buffer).unwrap_or_else(|err| {
            eprintln!("Error reading file `{}`: {}", filename, err);
            process::exit(1);
        });
        if n == 0 {
            break;
        }
        let s = str::from_utf8(&buffer[0..n]).unwrap_or_else(|err| {
            eprintln!("Error while reading file `{}`: {}", filename, err);
            process::exit(1);
        });
        print!("{}", s);
    }
}
