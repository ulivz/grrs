use anyhow::{Result};

mod cli;
mod fs;
mod progress;
mod stdout;

fn main() -> Result<()> {
    // progress::bar();
    // stdout::log();
    let cli = cli::instance();
    let content = fs::read_file(&cli.path)?;
    println!("file content: {}", content);
    for line in content.lines() {
        if line.contains(&cli.pattern) {
            println!("{}", line)
        }
    }

    Ok(())
}
