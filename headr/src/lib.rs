use std::{
    fs::File, 
    io::{self, BufReader, BufRead}
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, version, author)]
pub struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(
        short('n'),
        long,
        help("print the first NUM lines instead of the first 10;
        with the leading '-', print all but the last
        NUM lines of each file"),
        conflicts_with("bytes"),
        default_value("10"),
        value_parser(clap::value_parser!(u64).range(1..))
    )]
    lines: u64,

    #[arg(
        short('c'),
        long,
        help("print the last NUM bytes of each file;
        with the leading '-', print all but the last
        NUM bytes of each file"),
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

pub fn run(args: Args) -> Result<()> {
    // dbg!(args);
    for (file_num, filename) in args.files.iter().enumerate() {
        let flag = 
            if args.files.len() > 1 { true } else { false };

        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if flag {
                    println!("==> {} <==", &filename);
                }

                match args.bytes {
                    Some(bytes) => {
                        let mut buf = vec![0; bytes as usize];
                        let bytes_read = file.read(&mut buf)?;
                        println!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                    },
                    _ => {
                        for line in file.lines().take(args.lines as usize) {
                            println!("{}", line?);
                        }
                        if file_num < args.files.len() - 1 {
                            println!();
                        }
                    } 
                }
            }
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> Result<Box<dyn std::io::BufRead>> {
    match filename {
        "-" => Ok(
            Box::new(BufReader::new(io::stdin()))
        ),
        _ => Ok(
            Box::new(BufReader::new(File::open(filename)?))
        ),
    }
}

