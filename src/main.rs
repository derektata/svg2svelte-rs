// src/main.rs
use clap::Parser;
use std::path::PathBuf;
use svg2svelte::Processor;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    svg_file: PathBuf,

    #[clap(short, long)]
    typescript: bool,

    #[clap(short, long)]
    verbose: bool,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    Processor::new(cli.svg_file)
        .typescript(cli.typescript)
        .verbose(cli.verbose)
        .run()
}
