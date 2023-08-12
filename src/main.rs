use aheui::AheuiGrid;
use clap::Parser;

/// Aheui interpreter & compiler
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Source path
    input: String,
}

fn main() {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.input).expect("Could not read from input file");
    println!("{:?}", AheuiGrid::new(source));
}
