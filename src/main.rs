use clap::Parser;
use svg2svelte::svg_file::process;

/// Easily convert an SVG file into a Svelte Component, Rewritten in Rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// File to be processed
    svg_file: Option<String>,

    /// Create a Typescript component
    #[clap(short, long)]
    typescript: bool,

    /// Print the generated component to stdout
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    let svg_file = cli.svg_file.unwrap();
    process(&svg_file, cli.typescript, cli.verbose);
}
