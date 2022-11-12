use clap::{Parser};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), Box<CustomError>> {
    let args = Cli::parse();
    println!("args {:?}", args);
    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
    println!("file content: {}", content);
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
    Ok(())
}
