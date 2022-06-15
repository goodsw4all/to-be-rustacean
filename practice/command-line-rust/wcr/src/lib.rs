use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// impl BufRead : the file value must implement the BufRead trait
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    for byte in &buf {
        if *byte == '\n' as u8 {
            num_lines += 1;
        }
    }

    let num_words = match std::str::from_utf8(&buf) {
        Ok(v) => v.split_whitespace().count(),
        Err(_e) => 0,
    };

    let num_bytes = buf.len();
    let num_chars = match std::str::from_utf8(&buf) {
        Ok(v) => v.chars().count(),
        Err(_e) => 0,
    };

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("MW")
        .about("Rust wc")
        .arg(
            // postional argument
            Arg::new("files")
                .value_name("FILES")
                .default_value("-")
                .allow_invalid_utf8(true)
                .multiple_values(true)
                .help("Input Files"),
        )
        .arg(
            Arg::new("lines")
                .help("Show line count")
                .takes_value(false)
                .short('l')
                .long("lines"),
        )
        .arg(
            Arg::new("words")
                .help("Show word count")
                .takes_value(false)
                .short('w')
                .long("words"),
        )
        .arg(
            Arg::new("bytes")
                .help("Show byte count")
                .takes_value(false)
                .conflicts_with("chars")
                .short('c')
                .long("bytes"),
        )
        .arg(
            Arg::new("chars")
                .help("Show character count")
                .takes_value(false)
                .short('m')
                .long("chars"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = (0, 0, 0, 0);

    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(fh) => {
                let counted_data = count(fh);
                if let Ok(data) = counted_data {
                    let mut output = String::new();

                    let output_num_lines = if config.lines {
                        format!("{:>8}", data.num_lines)
                    } else {
                        format!("")
                    };
                    let output_num_words = if config.words {
                        format!("{:>8}", data.num_words)
                    } else {
                        format!("")
                    };
                    let output_num_bytes = if config.bytes {
                        format!("{:>8}", data.num_bytes)
                    } else {
                        format!("")
                    };
                    let output_num_chars = if config.chars {
                        format!("{:>8}", data.num_chars)
                    } else {
                        format!("")
                    };

                    total.0 += data.num_lines;
                    total.1 += data.num_words;
                    total.2 += data.num_bytes;
                    total.3 += data.num_chars;
                    [
                        output_num_lines,
                        output_num_words,
                        output_num_bytes,
                        output_num_chars,
                    ]
                    .iter()
                    .for_each(|result| output += result);

                    if filename == "-" {
                        println!("{}", output);
                    } else {
                        println!("{} {}", output, filename);
                    };
                }
            }
        }
    }
    
    if config.files.len() > 1 {
        let total_num_lines = if config.lines {
            format!("{:>8}", total.0)
        } else {
            format!("")
        };
        let total_num_words = if config.words {
            format!("{:>8}", total.1)
        } else {
            format!("")
        };
        let total_num_bytes = if config.bytes {
            format!("{:>8}", total.2)
        } else {
            format!("")
        };
        let total_num_chars = if config.chars {
            format!("{:>8}", total.3)
        } else {
            format!("")
        };
        println!("{total_num_lines}{total_num_words}{total_num_bytes}{total_num_chars} total");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
