use core::{compute_error, find_ratio};

use clap::Parser;

mod core;

/// Simple program to find the rational approximation of a decimal value.
#[derive(Parser, Default, Debug)]
#[command(version, about)]
struct Args {
    /// The target decimal number to approximate.
    target: f64,

    /// The maximum allowed absolute error.
    #[arg(default_value_t = 1e-3)]
    max_error: f64,

    /// The maximum allowed denominator.
    #[arg(default_value_t = 10000)]
    max_denominator: u32,
}

fn main() {
    let args = Args::parse();
    match find_ratio(args.target, args.max_error, args.max_denominator)
    {
        Some(ratio) => println!("An approximation of {} with an error of {} is {}", args.target, compute_error(ratio, args.target), ratio),
        None => println!("Could not find approximation of {} with an error below {} and a denominator below {}.", args.target, args.max_error, args.max_denominator),
    }
}
