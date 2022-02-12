//! head - outputs the first part of files

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::str;

#[derive(Copy, Clone)]
enum ToRead {
    NumBytes(i64),
    NumLines(i64),
}

fn main() {
    let matches = App::new("rust-head")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Prints the first part of each given file.")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .takes_value(true)
                .value_name("NUM")
                .help("Print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file")
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .takes_value(true)
                .value_name("NUM")
                .default_value("10")
                .help("Print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file")
        )
        .arg(
            Arg::new("zero")
                .short('z')
                .long("zero-terminated")
                .takes_value(false)
                .help("Delimit 'lines' with a zero byte instead of line feed.")
        )
        .arg(
            Arg::new("files")
                .required(false)
                .multiple_occurrences(true)
                .default_value("-")
        )
        .get_matches();

    let files: Vec<&str> = matches.values_of("files").unwrap().collect();

    let to_read = if matches.is_present("bytes") {
        let val = matches.value_of("bytes").unwrap();
        if val.chars().next().unwrap() == '-' {
            panic!("Leading `-' is unsupported")
        }
        let c = val.parse::<i64>().unwrap();
        ToRead::NumBytes(c)
    } else {
        let val = matches.value_of("lines").unwrap();
        if val.chars().next().unwrap() == '-' {
            panic!("Leading `-' is unsupported")
        }
        let n = val.parse::<i64>().unwrap();
        ToRead::NumLines(n)
    };

    for filename in files {
        // Open the file
        let reader: Box<dyn io::BufRead> = match filename.as_ref() {
            "-" => Box::new(io::BufReader::new(io::stdin())),
            filename => Box::new(io::BufReader::new(File::open(filename).unwrap())),
        };

        head(reader, to_read, '\n' as u8).expect("Read lines");
    }
}

fn head(mut reader: Box<dyn io::BufRead>, to_read: ToRead, delim: u8) -> std::io::Result<()> {
    match to_read {
        ToRead::NumBytes(nbytes) => {
            let bytes = nbytes as usize;

            let mut written: usize = 0;
            while written < bytes {
                // Read until the delimiter
                let mut line = vec![];
                reader.read_until(delim, &mut line)?;
                // Then write the number of bytes in the line or the
                // number remaining in our alloted balance. Whichever
                // is fewer.
                if line.len() < bytes - written {
                    written += line.len();
                    print!("{}", str::from_utf8(&line).unwrap());
                } else {
                    let short = line.get(0..(bytes - written)).unwrap();
                    written += short.len();
                    print!("{}", str::from_utf8(&short).unwrap());
                }
            }

            Ok(())
        }
        ToRead::NumLines(nlines) => {
            for _i in 0..nlines {
                let mut line = vec![];
                reader.read_until(delim, &mut line)?;
                print!("{}", str::from_utf8(&line).unwrap());
            }
            Ok(())
        }
    }
}
