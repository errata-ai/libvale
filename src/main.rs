use clap::Parser;

/// A helper utility for running Vale in CI environments.
#[derive(Parser, Debug)]
#[command(version)]
struct Args;

fn main() {
    let _ = Args::parse();
    println!("Hello, world!");
}
