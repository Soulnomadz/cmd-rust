use clap::Parser;
use headr::*;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
