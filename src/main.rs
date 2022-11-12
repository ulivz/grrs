use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("args {:?}", args);
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => {
            println!("file content: {}", content);
            for line in content.lines() {
                if line.contains(&args.pattern) {
                    println!("{}", line)
                }
            }
        }
        Err(error) => {
            println!("Oh noes:: {}", error)
        }
    }
}
