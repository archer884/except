use std::{
    fs,
    io::{self, BufRead, Write},
    process,
};

use clap::Parser;
use hashbrown::HashSet;

/// filter out strings found in a file
#[derive(Clone, Debug, Parser)]
struct Args {
    path: String,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: Args) -> io::Result<()> {
    let filter_text = fs::read_to_string(&args.path)?;
    let filter: HashSet<_> = filter_text.lines().collect();

    let mut out = io::stdout().lock();

    for s in io::stdin().lock().lines() {
        let s = s?;
        if !filter.contains(&*s) {
            writeln!(out, "{s}")?;
        }
    }

    Ok(())
}
