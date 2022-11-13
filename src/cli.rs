use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    pub pattern: String,
    pub path: std::path::PathBuf,
}

pub fn instance() -> Cli {
    let cli = Cli::parse();
    println!("cli {:?}", cli);
    cli
}
