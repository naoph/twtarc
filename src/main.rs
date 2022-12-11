mod ap;

use clap::Parser;

fn main() {
    let args = ap::Cli::parse();
    eprintln!("{:#?}", args);
}
