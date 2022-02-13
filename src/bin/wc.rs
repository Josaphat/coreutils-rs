//! wc - word, line, and byte or character count

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Counts {
    bytes: usize,
    chars: usize,
    words: usize,
    newlines: usize,
}

fn main() {
    let matches = App::new("rust-wc")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Rust clone of the wc utility. Counts the number of bytes, characters, words, ands newlines in each given FILE, or standard input if none are given or for a FILE of `-'.  A word is a nonzero length sequence of printable characters delimited by white space.")
        .arg(
            Arg::new("file")
                .help("A pathname of an input file. If none is specified or the filename is `-', then standard input is used.")
                .required(false)
                .default_value("-")
                .multiple_occurrences(true)
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .takes_value(false)
                .help("Print only byte counts")
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .takes_value(false)
                .help("Print only character counts")
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .takes_value(false)
                .help("Print only the word counts")
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .takes_value(false)
                .help("Print only the newline character counts")
        )
        .after_help("This application is free software.")
        .get_matches();

    let bytes_only = matches.is_present("bytes");
    let chars_only = matches.is_present("chars");
    let words_only = matches.is_present("words");
    let lines_only = matches.is_present("lines");

    let files: Vec<_> = matches.values_of("file").unwrap().collect();
    let files_len = files.len();

    let mut files_counts = vec![];

    let mut bytes_tot = 0;
    let mut chars_tot = 0;
    let mut words_tot = 0;
    let mut newlines_tot = 0;

    for filename in files {
        let reader: Box<dyn BufRead> = if filename == "-" {
            Box::new(BufReader::new(std::io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        };
        let res = wordcount(reader);

        bytes_tot += res.bytes;
        chars_tot += res.chars;
        words_tot += res.words;
        newlines_tot += res.newlines;

        files_counts.push((filename, res));
    }

    // Use the total for the bytes and make the width of the columns
    // all equal.  Bytes is the smallest unit, so it's going to be the
    // largest number.
    let col_width = bytes_tot.to_string().len();

    for counts in files_counts {
        if bytes_only || chars_only || words_only || lines_only {
            if bytes_only {
                print!("{:1$} ", counts.1.bytes, col_width);
            }
            if chars_only {
                print!("{:1$} ", counts.1.chars, col_width);
            }
            if words_only {
                print!("{:1$} ", counts.1.words, col_width);
            }
            if lines_only {
                print!("{:1$} ", counts.1.newlines, col_width);
            }
            println!("{}", if counts.0 != "-" { counts.0 } else { "" });
        } else {
            println!(
                "{:4$} {:4$} {:4$} {}",
                counts.1.newlines,
                counts.1.words,
                counts.1.bytes,
                if counts.0 != "-" { counts.0 } else { "" },
                col_width
            );
        }
    }

    if files_len > 1 {
        if bytes_only || chars_only || words_only || lines_only {
            if bytes_only {
                print!("{:1$} ", bytes_tot, col_width);
            }
            if chars_only {
                print!("{:1$} ", chars_tot, col_width);
            }
            if words_only {
                print!("{:1$} ", words_tot, col_width);
            }
            if lines_only {
                print!("{:1$} ", newlines_tot, col_width);
            }
            println!("{}", "total");
        } else {
            println!(
                "{:4$} {:4$} {:4$} {}",
                newlines_tot, words_tot, bytes_tot, "total", col_width
            );
        }
    }
}

fn wordcount(mut reader: Box<dyn BufRead>) -> Counts {
    let mut bytes = 0;
    let mut chars = 0;
    let mut words = 0;
    let mut newlines = 0;

    loop {
        let mut buf = String::new();
        let num_bytes = reader
            .read_line(&mut buf)
            .expect("Error reading text from file");
        bytes += num_bytes;
        if num_bytes == 0 {
            // Reached EOF
            break;
        }
        chars += buf.len();
        let mut in_word = false;
        for c in buf.bytes() {
            let is_whitespace = char::is_ascii_whitespace(&(c as char));
            if in_word && is_whitespace {
                in_word = false;
                words += 1;
            }
            if !in_word && !is_whitespace {
                in_word = true;
            }
            if c == b'\n' {
                newlines += 1;
            }
        }
        // consider end of line the end of the word
        if in_word {
            words += 1;
        }
    }

    Counts {
        bytes: bytes,
        chars: chars,
        words: words,
        newlines: newlines,
    }
}
