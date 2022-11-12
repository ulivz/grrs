use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("args {:?}", args);
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };
    println!("file content: {}", content);
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
    Ok(())
}
