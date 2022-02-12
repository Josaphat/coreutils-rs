//! yes - output a string repeatedly until killed

use clap::{App, Arg};
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    let matches = App::new("rust-yes")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Prints the given arguments forever until killed.")
        .arg(
            Arg::new("string")
                .value_name("STRING")
                .default_value("y")
                .multiple_occurrences(true),
        )
        .get_matches();
    let string = matches
        .values_of("string")
        .unwrap()
        .collect::<Vec<&str>>()
        .join(" ");

    // We'd like to just use the println! macro, but this doesn't
    // exhibit graceful behavior when part of a pipeline that gets
    // closed.
    // We'll use the write! family of macros instead, quietly exiting
    // the program in case of errors.
    let mut out = io::stdout();
    loop {
        writeln!(out, "{}", string).unwrap_or_else(|_| {
            process::exit(0);
        });
    }
}
