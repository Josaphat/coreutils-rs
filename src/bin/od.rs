//! od - dump files in various formats

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read};
use std::process;
use std::vec;

enum AddressRadix {
    Octal,
    Hexadecimal,
    Decimal,
    None,
}

enum Endian {
    Little,
    Big,
}

enum FormatType {
    NamedCharacter,
    PrintableCharacter,
    SignedDecimal,
    Octal,
    UnsignedDecimal,
    Hexadecimal,
}

enum CharacterWidth {
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    LongDouble,
}

struct Format {
    ftype: FormatType,
    character_width: i32,
    is_display: bool,
}

fn parse_i16(a: u8, b: u8, endian: Endian) -> i16 {
    let x: i16 = a.into();
    let y: i16 = b.into();

    match endian {
        Endian::Little => x << 0 | y << 8,
        Endian::Big => y << 0 | x << 8,
    }
}

fn main() {
    let matches = App::new("rust-od")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Rust clone of the od utility.")
        .arg(
            Arg::with_name("FILE")
                .help("Input file to written to standard output in given formats")
                .default_value("-")
                .required(false)
                .multiple(true),
        )
        .get_matches();

    let address_radix = AddressRadix::Octal;
    let width = 16; // number of bytes on a line.
    let output_duplicates = true;

    // The default format, if unspecified, is "oS". On a platform
    // where a 'short' is 16 bits, this is the same as "o2".
    // let formats = vec![Format{ftype: FormatType::Octal, character_width: 2, is_display: false}];
    let mut offset = 0;

    let mut newline = false;

    // Unwrap is fine here; FILE will have a default.
    let files: Vec<_> = matches.values_of("FILE").unwrap().collect();
    'fileloop: for filename in files {
        let mut reader: Box<dyn io::Read> = if filename == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(File::open(filename).unwrap_or_else(|err| {
                eprintln!("Error reading file `{}`: {}", filename, err);
                process::exit(1);
            }))
        };

        let mut eof = false;
        while !eof {
            print!("{:07o}", offset);

            // Read an integer
            for i in 0..(width / 2) {
                let mut int_bytes = vec![0; 2];
                let n = reader.read(&mut int_bytes).unwrap();
                if n == 0 {
                    // EOF. Go to next file.
                    eof = true;
                    break;
                }

                let int = parse_i16(
                    int_bytes[0],
                    if n > 1 { int_bytes[1] } else { 0 },
                    Endian::Little,
                );

                print!(" {:06o}", int);

                offset += n;
            }
            println!("");
        }
    }
}
