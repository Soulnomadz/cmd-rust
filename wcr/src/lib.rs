use clap::Parser;
use anyhow::Result;

use std::io::{BufRead, BufReader, self};

pub fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars].iter()
        .all(|v| v == &false) {
            args.lines = true;
            args.words = true;
            args.bytes = true;
        }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    
    // let num_file = &args.files.len();

    for filename in &args.files {
        let file = open(&filename)?;
        if let Ok(info) = count(file) {
            // println!("{} {} {} {}", info.num_lines, info.num_words, info.num_bytes, filename);
            // println!("{:>8} {:>8} {:>8} {}", info.num_lines, info.num_words, info.num_bytes, filename);
            println!("{}{}{}{}{}",
                showable(info.num_lines, args.lines),
                showable(info.num_words, args.words),
                showable(info.num_bytes, args.bytes),
                showable(info.num_chars, args.chars),
                if filename == "-" {
                    "".to_string()
                } else {
                    format!(" {filename}")
                }
            );
            
            total_lines += info.num_lines;
            total_words += info.num_words;
            total_bytes += info.num_bytes;
            total_chars += info.num_chars;
        }
    }

    if args.files.len() > 1usize {
        println!("{}{}{}{}{}",
            showable(total_lines, args.lines),
            showable(total_words, args.words),
            showable(total_bytes, args.bytes),
            showable(total_chars, args.chars),
            " total",
        );
    }    

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Rust version of wc from oywz
pub struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with = "bytes")]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: u64,
    num_words: u64,
    num_bytes: u64,
    num_chars: u64,
}

// impl FileInfo {
//     fn new() -> Self {
//         FileInfo {
//             num_lines: 0,
//             num_words: 0,
//             num_bytes: 0,
//             num_chars: 0,
//         }
//     }
// }
// -------------------------------------------------------------------------------
pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    loop {
        let line_bytes = file.read_line(&mut buf)?;
        if line_bytes == 0 { break; }

        num_lines += 1;
        num_words += buf.split_whitespace().count() as u64;
        num_bytes += buf.len() as u64;
        num_chars += buf.chars().count() as u64;
        
        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// -------------------------------------------------------------------------------
fn showable(value: u64, showable: bool) -> String {
    if showable {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}


// -------------------------------------------------------------------------------
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(
            Box::new(BufReader::new(io::stdin()))
        ),
        _ => Ok(
            Box::new(BufReader::new(std::fs::File::open(filename)?))
        ),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        
        assert!(info.is_ok());
        
        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}