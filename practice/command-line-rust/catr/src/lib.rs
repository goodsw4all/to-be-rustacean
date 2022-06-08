use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("MW")
        .about("rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .allow_invalid_utf8(true)
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number the output lines")
                .takes_value(false), // means it's a flag
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number the non-blank output lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        // files: matches.values_of_lossy("files").unwrap_or(vec![]),
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_number = 0;
                for (idx, line) in file.lines().enumerate() {
                    if config.number_lines {
                        println!("{:>6}\t{}", idx + 1, line.unwrap());
                    } else if config.number_nonblank_lines {
                        let text = line.unwrap();
                        if text == "" {
                            println!("");
                        } else {
                            line_number += 1;
                            println!("{:>6}\t{}", line_number, text);
                        }
                    } else {
                        println!("{}", line.unwrap());
                    }
                }
            }
        }
    }
    // dbg!(config);
    Ok(())
}
