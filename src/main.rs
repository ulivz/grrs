use anyhow::{Context, Result};
use clap::Parser;

mod progress;
mod stdout;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // progress::bar();
    stdout::log();

    let args = Cli::parse();
    println!("args {:?}", args);
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", &args.path.display()))?;
    println!("file content: {}", content);
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
    Ok(())
}
