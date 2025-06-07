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



fn main() {
    let cli = Cli::parse(); // This parses arguments, handles errors, and exits if invalid

    // Now, `cli` contains your parsed arguments:
    println!("Query: {}", cli.query);
    println!("File Path: {}", cli.file_path);
    println!("Ignore Case: {}", cli.ignore_case);

    // Your existing run function will eventually use these.
    // For now, just focus on parsing and printing `cli`.
}
