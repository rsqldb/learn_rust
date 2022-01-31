use std::env;
use std::process;
use grep::Config;

// command:  `cargo run drew input_file.txt`
// case insensitive command: `CASE_INSENSITIVE=1 cargo run Drew input_file.txt`
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Failed to parse config: {}", error);
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        eprintln!("Failed to read file: {}", e);
        process::exit(1);
    }
}

