mod cli;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("flockd v{}", env!("CARGO_PKG_VERSION"));
    println!("log_level={}", cli.log_level);
    println!("config={}", cli.config.display());
}
