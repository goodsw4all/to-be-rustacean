use clap::{Arg, Command};
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .about("rust uniqr")
        .author("MW")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input File")
                .allow_invalid_utf8(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .allow_invalid_utf8(true)
                .help("Output File"),
        )
        .arg(
            Arg::new("count")
                .help("Show counts")
                .short('c')
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches
            .value_of_lossy("in_file")
            .map(|s| s.to_string())
            .unwrap(),
        out_file: matches.value_of_lossy("out_file").map(|s| s.to_string()),
        count: matches.is_present("count"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    //Err(err) => eprintln!("{}: {}", filename, err),
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut current_line = String::new();
    let mut to_print_line = String::new();
    let mut count = 0;

    let mut buffer: Box<dyn Write> = match &config.out_file {
        Some(out) => Box::new(fs::File::create(out)?),
        _ => Box::new(io::stdout()),
    };

    loop {
        let bytes = file.read_line(&mut current_line)?;
        if bytes == 0 {
            break;
        }

        if to_print_line.trim() == current_line.trim() {
            count += 1;

            to_print_line = current_line.clone();
            current_line.clear();
            continue;
        }

        if count > 0 {
            if config.count {
                write!(buffer, "{:4} {}", count, to_print_line)?;
            } else {
                buffer.write(to_print_line.as_bytes())?;
            }
            count = 0;
        } else {
        }

        to_print_line = current_line.clone();
        current_line.clear();
        count += 1;
    }

    // last line
    if count > 0 {
        if config.count {
            write!(buffer, "{:4} {}", count, to_print_line)?;
        } else {
            buffer.write(to_print_line.as_bytes())?;
        }
    }

    Ok(())
}

// pub fn run(config: Config) -> MyResult<()> {
//     //Err(err) => eprintln!("{}: {}", filename, err),
//     let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

//     let mut current_line = String::new();
//     let mut to_print_line = String::new();
//     let mut count = 0;

//     let mut buffer: Box<dyn Write> = match &config.out_file {
//         Some(out) => Box::new(fs::File::create(out)?),
//         _ => Box::new(io::stdout()),
//     };

//     loop {
//         let bytes = file.read_line(&mut current_line)?;
//         if bytes == 0 {
//             break;
//         }

//         if to_print_line.trim() != current_line.trim() {
//             if count > 0 {
//                 if config.count {
//                     write!(buffer, "{:4} {}", count, to_print_line)?;
//                 } else {
//                     buffer.write(to_print_line.as_bytes())?;
//                 }
//             }

//             to_print_line = current_line.clone();
//             count = 0;
//         }

//         count += 1;
//         current_line.clear();
//     }

//     // println!("Out of loop");
//     if count > 0 {
//         // last line
//         if config.count {
//             write!(buffer, "{:4} {}", count, to_print_line)?;
//         } else {
//             buffer.write(to_print_line.as_bytes())?;
//         }
//     }

//     Ok(())
// }
