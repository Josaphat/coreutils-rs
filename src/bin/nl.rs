//! nl - line numbering filter, based on GNU coreutils implementation of nl.

use clap::{App, Arg};
use std::collections;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    let matches = App::new("rust-nl")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Rust clone of the nl utility.\nNOTE: It is missing a few things, including the `pBRE` STYLE option. See documentation for more details.")
        .arg(
            Arg::new("body_style")
                .short('b')
                .long("body-numbering")
                .takes_value(true)
                .value_name("STYLE")
                .help("use STYLE for numbering body lines")
                .default_value("t"),
        )
        .arg(
            Arg::new("section_delim")
                .short('d')
                .long("section-delimiter")
                .takes_value(true)
                .value_name("CC")
                .help("(CURRENTLY IGNORED) use CC for logical page delimiters"),
        )
        .arg(
            Arg::new("footer_style")
                .short('f')
                .long("footer-numbering")
                .takes_value(true)
                .value_name("STYLE")
                .help("use STYLE for numbering footer lines")
                .default_value("n"),
        )
        .arg(
            Arg::new("header_style")
                .short('h')
                .long("header-numbering")
                .takes_value(true)
                .value_name("STYLE")
                .help("use STYLE for numbering header lines")
                .default_value("n"),
        )
        .arg(
            Arg::new("line_increment")
                .short('i')
                .long("line-increment")
                .takes_value(true)
                .value_name("NUMBER")
                .help("line number increment at each line")
                .default_value("1"),
        )
        .arg(
            Arg::new("number_format")
                .short('n')
                .long("number-format")
                .value_name("FORMAT")
                .help("insert line numbers according to FORMAT")
                .default_value("rn"),
        )
        .arg(
            Arg::new("no_renumber")
                .short('p')
                .long("no-renumber")
                .help("do not reset line numbers for each section")
                .takes_value(false),
        )
        .arg(
            Arg::new("number_separator")
                .short('s')
                .long("number-separator")
                .value_name("STRING")
                .help("add STRING after (possible) line number")
                .default_value("\t"),
        )
        .arg(
            Arg::new("starting_line_number")
                .short('v')
                .long("starting-line-number")
                .value_name("NUMBER")
                .help("first line number for each section")
                .default_value("1"),
        )
        .arg(
            Arg::new("number_width")
                .short('w')
                .long("number-width")
                .value_name("NUMBER")
                .help("use NUMBER columns for line numbers")
                .default_value("6"),
        )
        .arg(
            Arg::new("file")
                .help("A pathname of a text file to be line-numbered.")
                .required(false),
        )
        .after_help("Default options are: -bt -d'\\:' -fn -hn -i1 -l1 -n'rn' -s<TAB> -v1 -w6\n\n\
                     CC are two delimiter characters used to construct logical page delimiters; a missing second character implies ':'.\n\n\
                     STYLE is one of:\n\n\
                     \ta\t\tnumber all lines\n\
                     \tt\t\tnumber only nonempty lines\n\
                     \tn\t\tnumber no lines\n\
                     \tpBRE\t\tnumber only lines that contain a match for the basic regular expression, BRE (currently ignored)\n\
                     \nFORMAT is one of:\n\n\
                     \tln\t\tleft justified, no leading zeros\n\
                     \trn\t\tright justified, no leading zeros\n\
                     \trz\t\tright justified, leading zeros\n")
        .get_matches();

    let no_renumber = matches.is_present("no_renumber");

    let body_style = matches.value_of("body_style").unwrap_or_else(|| {
        eprintln!("Could not parse body style parameter");
        process::exit(1);
    });

    // Ignoring section_delim...

    let header_style = matches.value_of("header_style").unwrap_or_else(|| {
        eprintln!("Could not parse header style parameter");
        process::exit(1);
    });

    let footer_style = matches.value_of("footer_style").unwrap_or_else(|| {
        eprintln!("Could not parse footer style parameter");
        process::exit(1);
    });

    let number_format = matches.value_of("number_format").unwrap_or_else(|| {
        eprintln!("Could not parse -n, --number-format=FORMAT");
        process::exit(1);
    });
    if number_format != "rn" && number_format != "rz" && number_format != "ln" {
        eprintln!("Invalid values for -n, --number-format=FORMAT");
        eprintln!("Valid values are\n\t'ln'    left-justified, no leading zeroes;\n\t'rn'    right-justified, no leading zeroes;\n\t'rz'      right-justified, leading zeroes.");
        process::exit(1);
    }

    let number_separator = matches.value_of("number_separator").unwrap_or_else(|| {
        eprintln!("Could not parse -s, --number-separator=STRING");
        process::exit(1);
    });

    let starting_line_str = matches.value_of("starting_line_number").unwrap_or_else(|| {
        eprintln!("Could not parse -v, --starting-line-number=NUMBER");
        process::exit(1);
    });

    let starting_line_number = starting_line_str.parse::<u32>().unwrap_or_else(|err| {
        eprintln!(
            "Invalid starting line number: '{}'\nError: {}",
            starting_line_str, err
        );
        eprintln!("usage: -v, --starting-line-number=NUMBER");
        process::exit(1);
    });

    let line_increment = matches
        .value_of("line_increment")
        .unwrap()
        .parse::<u32>()
        .expect("Invalid value for -i, --line-increment=NUMBER");

    let number_width = matches
        .value_of("number_width")
        .unwrap()
        .parse::<usize>()
        .expect("Invalid value for -w, --number-width=NUMBER");

    let filename = matches.value_of("file").unwrap_or("-");
    let reader: Box<dyn io::BufRead> = match filename {
        "-" => Box::new(io::BufReader::new(io::stdin())),
        filename => Box::new(io::BufReader::new(fs::File::open(filename).unwrap())),
    };

    nl(
        no_renumber,
        header_style,
        body_style,
        footer_style,
        number_format,
        number_separator,
        starting_line_number,
        line_increment,
        number_width,
        reader,
    );
}

fn nl(
    no_renumber: bool,
    header_style: &str,
    body_style: &str,
    footer_style: &str,
    number_format: &str,
    number_separator: &str,
    starting_line_number: u32,
    line_increment: u32,
    number_width: usize,
    reader: Box<dyn io::BufRead>,
) {
    let mut line_count = starting_line_number;
    let mut section = "BODY";
    let map: collections::HashMap<&str, &str> = [
        ("HEADER", header_style),
        ("BODY", body_style),
        ("FOOTER", footer_style),
    ]
    .iter()
    .cloned()
    .collect();

    for line in reader.lines() {
        let line = line.unwrap_or_default();
        let style = map[section];

        if line == "\\:\\:\\:" {
            // The delimiter line is considered empty.
            println!("{}", "");
            // Reset the line count
            if !no_renumber {
                line_count = starting_line_number;
            }
            // This is the beginning of a header
            section = "HEADER";
            continue;
        }
        if line == "\\:\\:" {
            println!("{}", "");
            // Reset the line count
            if !no_renumber {
                line_count = starting_line_number;
            }
            section = "BODY";
            continue;
        }
        if line == "\\:" {
            println!("{}", "");
            // Reset the line count
            if !no_renumber {
                line_count = starting_line_number;
            }
            section = "FOOTER";
            continue;
        }

        if style == "t" || style == "a" {
            if line == "" && style == "t" {
                // Do not number blank lines
                println!("{}", line);
            } else {
                // We can't dynamically set alignment like we can
                // width (AFAICT).  The only difference between these
                // three arms is the alignment field (either ">",
                // "0>", or "<").
                match number_format {
                    "rn" => println!(
                        "{:>width$}{}{}",
                        line_count,
                        number_separator,
                        line,
                        width = number_width
                    ),
                    "rz" => println!(
                        "{:0>width$}{}{}",
                        line_count,
                        number_separator,
                        line,
                        width = number_width
                    ),
                    "ln" => println!(
                        "{:<width$}{}{}",
                        line_count,
                        number_separator,
                        line,
                        width = number_width
                    ),
                    _ => unreachable!(),
                }
                line_count += line_increment;
            }
        } else if style == "n" {
            // No numbering. Only printing.  Account for the width of
            // the separator. We want the outputs to line up with
            // non-numbered sections.
            let indent = " ".repeat(number_width + number_separator.len());
            println!("{}{}", indent, line);
        }
    }
}
