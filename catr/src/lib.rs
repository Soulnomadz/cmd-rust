use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    println!("{:?}", get_args());
    Ok(())
}

fn get_args() -> Result<Config> {
    let args = Args::parse();
    Ok(Config {
        files: args.files,
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines,
    })
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
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
