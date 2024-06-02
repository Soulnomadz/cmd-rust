use anyhow::Result;
use clap::Parser;

use std::io::{self, BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,
    #[arg(
        short('b'),
        long("number-nonblank"),
    )]
    number_nonblank_lines: bool,
}


pub fn run(args: Args) -> Result<()> {
    // dbg!(config);
    for f in args.files {
        // dbg!(&f);
        match open(&f) {
            Err(err) => eprintln!("{}: {}", f, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if args.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if args.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(
            Box::new(BufReader::new(io::stdin()))
        ),
        _ => Ok(
            Box::new(BufReader::new(File::open(filename)?))
        ),
    }
}

// #[derive(Debug)]
// pub struct Config {
//     files: Vec<String>,
//     number_lines: bool,
//     number_nonblank_lines: bool,
// }

// pub fn get_args() -> Result<Config> {
//     let args = Args::parse();
//     Ok(Config {
//         files: args.files,
//         number_lines: args.number_lines,
//         number_nonblank_lines: args.number_nonblank_lines,
//     })
// }


