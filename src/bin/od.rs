//! od - dump files in various formats

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::Read;
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
    // Big,
}

// enum FormatType {
//     NamedCharacter,
//     PrintableCharacter,
//     SignedDecimal,
//     Octal,
//     UnsignedDecimal,
//     Hexadecimal,
// }

// enum CharacterWidth {
//     Char,
//     Short,
//     Int,
//     Long,
//     Float,
//     Double,
//     LongDouble,
// }

struct Format {
    // ftype: FormatType,
    character_width: usize,
    // is_display: bool,
}

fn parse_i16(a: u8, b: u8, endian: Endian) -> i16 {
    let x: i16 = a.into();
    let y: i16 = b.into();

    match endian {
        Endian::Little => x << 0 | y << 8,
        // Endian::Big => y << 0 | x << 8,
    }
}

fn main() {
    let matches = App::new("rust-od")
        .version("0.1.0")
        .author("Jos V. <jos@josaphat.co>")
        .about("Rust clone of the od utility. Very incomplete. Supports only the -A/--address-radix option")
        .arg(
            Arg::new("FILE")
                .help("Input file to written to standard output in given formats")
                .default_value("-")
                .required(false)
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("address_radix")
                           .short('A')
                           .long("address-radix")
                           .takes_value(true)
                           .value_name("radix")
                           .help("Select the base in which file offsets are printed. radix can be one of the following:\n\td - decimal,\n\to - octal,\n\tx - hexadecimal,\n\tn - none (do not print offsets).")
                           .default_value("o")
        )
        .get_matches();

    let address_radix = match matches.value_of("address_radix").unwrap_or_else(|| {
        eprintln!("invalid output address radix. must be one character from [doxn]");
        process::exit(1);
    }) {
        "d" => AddressRadix::Decimal,
        "x" => AddressRadix::Hexadecimal,
        "o" => AddressRadix::Octal,
        "n" => AddressRadix::None,
        x => {
            eprintln!(
                "invalid output address radix '{}'. Must be one character from [doxn]",
                x
            );
            process::exit(1);
        }
    };

    let width = 16; // number of bytes on a line.
    let output_duplicates = true;

    // The default format, if unspecified, is "oS". On a platform
    // where a 'short' is 16 bits, this is the same as "o2".
    // let formats = vec![Format{ftype: FormatType::Octal, character_width: 2, is_display: false}];
    let offset = 0;

    // Unwrap is fine here; FILE will have a default.
    let files: Vec<_> = matches.values_of("FILE").unwrap().collect();
    od(
        files,
        offset,
        output_duplicates,
        address_radix,
        Format {
            // ftype: FormatType::Octal,
            character_width: 2,
            // is_display: false,
        },
        width,
    )
}

// Instead of iterating over the files in a loop, we'll determine how
// many bytes to read from a line,
fn od(
    files: Vec<&str>,
    mut offset: usize,
    _output_duplicates: bool,
    addr_radix: AddressRadix,
    fmt: Format,
    width: usize,
) {
    let mut fs_iter = files.iter();

    let mut reader = open_reader(match fs_iter.next() {
        Some(x) => x,
        _ => return,
    });

    let mut end_of_input = false;
    loop {
        // Beginning of line
        match addr_radix {
            AddressRadix::Octal => print!("{:07o}", offset),
            AddressRadix::Hexadecimal => print!("{:06x}", offset),
            AddressRadix::Decimal => print!("{:07}", offset),
            AddressRadix::None => (),
        };

        // The GNU version of od appears to dump one final beginning
        // of line offest.
        if end_of_input {
            println!("");
            break;
        }

        let line_reads = width / fmt.character_width;

        for _i in 0..line_reads {
            let mut int_bytes = vec![0; 2];
            let mut n = reader.read(&mut int_bytes).unwrap();
            if n == 0 {
                // If we're at EOF, attempt to open the next file.
                // If there is no next file, we're done.
                reader = open_reader(match fs_iter.next() {
                    Some(x) => x,
                    _ => {
                        end_of_input = true;
                        break;
                    }
                });
                continue;
            } else if n < fmt.character_width {
                // I'm being a bit naughty here... This loop is just
                // so that I can break out if open_reader fails...
                loop {
                    reader = open_reader(match fs_iter.next() {
                        Some(x) => x,
                        _ => {
                            end_of_input = true;
                            break;
                        }
                    });
                    break;
                }

                if !end_of_input {
                    let mut bonus_byte = vec![0; 1];
                    let bonus_n = reader.read(&mut bonus_byte).unwrap();
                    if bonus_n == 0 {
                        // An error, I think. We just opened this file.
                        eprintln!("Error reading file");
                        process::exit(1);
                    }
                    n += bonus_n;
                    int_bytes[1] = bonus_byte[0];
                } else {
                    // We aren't going to get a full thing. Make the bonus byte a zero.
                    int_bytes[1] = 0; // should be redundant
                }
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

fn open_reader(filename: &str) -> Box<dyn io::Read> {
    if filename == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(filename).unwrap_or_else(|err| {
            eprintln!("Error reading file `{}`: {}", filename, err);
            process::exit(1);
        }))
    }
}
