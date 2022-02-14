//! wc - word, line, and byte or character count

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Stats {
    bytes: usize,
    chars: usize,
    words: usize,
    newlines: usize,
    max_line: usize,
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
        .arg(
            Arg::new("max_line_length")
                .short('L')
                .long("max-line-length")
                .help("print the maximum display width")
        )
        .after_help("This application is free software.")
        .get_matches();

    let bytes_arg = matches.is_present("bytes");
    let mut chars_arg = matches.is_present("chars");
    let mut words_arg = matches.is_present("words");
    let mut lines_arg = matches.is_present("lines");
    let max_line_arg = matches.is_present("max_line_length");

    // If none are specified it's as if '-clw' were specified
    if !bytes_arg && !chars_arg && !words_arg && !lines_arg && !max_line_arg {
        chars_arg = true;
        lines_arg = true;
        words_arg = true;
    }
    let chars_arg = chars_arg;
    let lines_arg = lines_arg;
    let words_arg = words_arg;

    let files: Vec<_> = matches.values_of("file").unwrap().collect();
    let files_len = files.len();

    let mut files_stats = vec![];

    let mut bytes_tot = 0;
    let mut chars_tot = 0;
    let mut words_tot = 0;
    let mut newlines_tot = 0;
    let mut maxest_line = 0;

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
        if res.max_line > maxest_line {
            maxest_line = res.max_line;
        }

        files_stats.push((filename, res));
    }
    if files_len > 1 {
        files_stats.push((
            "total",
            Stats {
                bytes: bytes_tot,
                chars: chars_tot,
                words: words_tot,
                newlines: newlines_tot,
                max_line: maxest_line,
            },
        ));
    }

    // Use the total for the bytes and make the width of the columns
    // all equal.  Bytes is the smallest unit, so it's going to be the
    // largest number.
    let col_width = bytes_tot.to_string().len();

    for stats in files_stats {
        if lines_arg {
            print!("{:1$} ", stats.1.newlines, col_width);
        }
        if words_arg {
            print!("{:1$} ", stats.1.words, col_width);
        }
        if chars_arg {
            print!("{:1$} ", stats.1.chars, col_width);
        }
        if bytes_arg {
            print!("{:1$} ", stats.1.bytes, col_width);
        }
        if max_line_arg {
            print!("{:1$} ", stats.1.max_line, col_width);
        }
        println!("{}", if stats.0 != "-" { stats.0 } else { "" });
    }
}

/// Computes and returns statistics for the text readable from the
/// given reader. The given reader is read until the EOF condition is
/// reached.
///
/// When reporting the number of characters, this does not take into
/// account "grapheme clusters," but it does consider individual code
/// points.
fn wordcount(mut reader: Box<dyn BufRead>) -> Stats {
    // Keep running counts of the values we care about
    let mut bytes = 0;
    let mut chars = 0;
    let mut words = 0;
    let mut newlines = 0;
    let mut max_line = 0;

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
        let mut in_word = false;
        let mut line_length = 0;
        for c in buf.chars() {
            chars += 1;
            line_length += 1;
            let is_whitespace = c.is_whitespace();
            if in_word && is_whitespace {
                in_word = false;
                words += 1;
            }
            if !in_word && !is_whitespace {
                in_word = true;
            }
            if c == '\n' {
                newlines += 1;
            }
            if c.is_control() {
                line_length -= 1;
            } else if c == '\u{feff}' || c == '\u{fffe}' {
                // These aren't printable...
                line_length -= 1;
            }
        }
        // consider end of line the end of the word
        if in_word {
            words += 1;
        }
        if line_length > max_line {
            max_line = line_length;
        }
    }

    Stats {
        bytes,
        chars,
        words,
        newlines,
        max_line,
    }
}
