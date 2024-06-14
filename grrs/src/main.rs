use clap::Parser;
use anyhow::{Result, Context};

use std::fs::File;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to search for
    pattern: String,
    /// The path to the file to search
    path: std::path::PathBuf,

    /* /// Verbosity
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    */
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    env_logger::init();

    // env_logger::Builder::new()
    //     .filter_level(args.verbose.log_level_filter())
    //     .init();

    log::info!("Starting to search");
    log::debug!("{:?}", &args);

    let buf = BufReader::new(
            File::open(&args.path)
                .with_context(|| format!("cannot read {}", &args.path.display()))?
        );

    grrs::find_matches(buf, &args.pattern, std::io::stdout())
}

