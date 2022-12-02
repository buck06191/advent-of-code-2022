use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("In file {:?}", args.path.as_path());

    if let Ok(lines) = read_lines(args.path.as_path()) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    };
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
