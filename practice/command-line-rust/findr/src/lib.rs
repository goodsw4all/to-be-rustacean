use crate::EntryType::*;
use clap::{Arg, Command};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("MW")
        .about("Rust Find")
        .arg(
            Arg::new("paths")
                .value_name("PATHS")
                .help("Paths to search")
                .default_value(".")
                .allow_invalid_utf8(true)
                .multiple_values(true),
        )
        .arg(
            Arg::new("names")
                .value_name("name pattern")
                .short('n')
                .long("name")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .allow_invalid_utf8(true)
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple_values(true),
        )
        .get_matches();

    let paths: Vec<String> = matches.values_of_lossy("paths").unwrap();
    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<Regex>, String>>()
        }) //.unwrap().unwrap();
        .transpose()? // Option<Result> -> Result<Option>
        .unwrap_or_default();

    // clap should disallow anything but "d," "f," or "l"
    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.path_is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                })
    };
    let name_filter = {
        |entry: &DirEntry| {
            config.names.is_empty()
                || config
                    .names
                    .iter()
                    .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
        }
    };

    for path in &config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}

pub fn run2(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprint!("{e}"),
                Ok(entry) => {
                    // types 인자가 비어있거나, 하나라도 참이면 println! 매크로 실행한다
                    if (config.entry_types.is_empty()
                        || config
                            .entry_types
                            .iter()
                            .any(|entry_type| match entry_type {
                                Link => entry.file_type().is_symlink(),
                                Dir => entry.file_type().is_dir(),
                                File => entry.file_type().is_file(),
                            }))
                        && (config.names.is_empty()
                            || config
                                .names
                                .iter()
                                .any(|re| re.is_match(&entry.file_name().to_string_lossy())))
                    {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())
}
