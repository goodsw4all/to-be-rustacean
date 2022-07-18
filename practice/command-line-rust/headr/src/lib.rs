use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .author("MW")
        .version("0.1.0")
        .about("Rust head")
        .arg( 
            Arg::new("files")
                .help("Input files")
                .value_name("FILES")
                .allow_invalid_utf8(true)
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("number of lines to show")
                .short('n')
                .long("lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("number of bytes to show")
                .short('c')
                .long("bytes")
                .conflicts_with("lines")
                .takes_value(true),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        // map for option instead of match
        // Maps an Option<T> to Option<U> by applying a function to a contained value.
        // Option has a built in method called map(),
        // a combinator for the simple mapping of Some -> Some
        // and None -> None. Multiple map() calls can be chained together for even more flexibility.
        .map(parse_positive_int)
        // Transposes an Option of a Result into a Result of an Option.
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        // _ => Err(Box::from(val)),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for (idx, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut bufread) => {
                if config.files.len() > 1 {
                    println!("==> {} <==", filename);
                }
                match config.bytes {
                    Some(n) => {
                        let mut handle = bufread.take(n as u64);
                        let mut buffer = vec![0; n];
                        let bytes_read = handle.read(&mut buffer)?;
                        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    }
                    None => {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = bufread.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    }
                }
                if config.files.len() > 1 && idx < config.files.len() - 1 {
                    println!();
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
