use std::{fs::File, io::{BufRead, BufReader}};

use clap::{Parser, command}; 

#[derive(Parser, Debug)] 
#[command(author, version, about, long_about = None)] // Add metadata for help messages
struct Cli {
    /// String to search for
    query: String,

    /// Path to the file to search in
    file_path: String,

    /// Ignore case when searching
    #[arg(short, long, default_value_t = false)]
    // short: -i, long: --ignore-case, default: false
    ignore_case: bool,
}

fn run(config: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&config.file_path)?;
    let reader = BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(line_str) => {
                if config.ignore_case {
                    if line_str
                        .to_lowercase()
                        .contains(&config.query.to_lowercase())
                    {
                        println!("Line {}: {line_str}", idx + 1);
                    }
                } else {
                    if line_str.contains(&config.query) {
                        println!("Line {}: {line_str}", idx + 1);
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse(); // This parses arguments, handles errors, and exits if invalid

    // Now, `cli` contains your parsed arguments:
    println!("Query: {}", cli.query);
    println!("File Path: {}", cli.file_path);
    println!("Ignore Case: {}", cli.ignore_case);
    if let Err(e) = run(cli) {
        eprintln!("Application error: {}", e);
        std::process::exit(1); // Exit with a non-zero status on error
    }
    
}
