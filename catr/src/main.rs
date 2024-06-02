use clap::Parser;
use catr::*;

fn main() {
    if let Err(e) = 
        run(Args::parse()) {
        // catr::get_args().and_then(catr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
