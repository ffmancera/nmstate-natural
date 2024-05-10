use clap::Parser;


/// Tool to generate natural language description from a Nmstate network state.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// YAML file path for the network state
    #[arg(short, long)]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
