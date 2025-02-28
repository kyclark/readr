use anyhow::{Result, anyhow};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};
use tokio::time::{Duration, sleep};

#[derive(Debug, Parser)]
struct Args {
    /// Input file
    #[arg()]
    file: String,

    /// Go slower
    #[arg(short, long)]
    slow: bool,

    /// Go faster
    #[arg(short, long, conflicts_with = "slow")]
    fast: bool,
}

// --------------------------------------------------
#[tokio::main]
async fn main() {
    if let Err(e) = run(Args::parse()).await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
async fn run(args: Args) -> Result<()> {
    let file = open(&args.file)?;
    let mult = if args.fast { 0.5} else if args.slow { 2.} else {1.};
    for line in file.lines().map_while(Result::ok) {
        for char in line.chars() {
            print!("{char}");
            let ms = if ".!?\n".contains(char) {
                500.
            } else if ",:;".contains(char) {
                200.
            } else {
                50.
            } * mult;
            std::io::stdout().flush()?;
            sleep(Duration::from_millis(ms as u64)).await;
        }
        println!();
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{filename}: {e}"))?,
        ))),
    }
}
