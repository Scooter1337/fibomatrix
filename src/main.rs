#![allow(clippy::needless_range_loop)]
use clap::Parser;
mod par;
mod single;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Length of the Fibonacci sequence to generate
    /// This is the number of Fibonacci numbers to generate
    /// The default is 10
    /// The minimum is 1
    #[clap(short, long)]
    length: usize,

    /// Parallel or single-threaded
    /// The default is parallel
    #[clap(long, short)]
    single_threaded: bool,

    /// Dont print the result
    /// The default is false
    #[clap(long, short)]
    quiet: bool,

    /// Time only
    /// The default is false
    #[clap(long, short)]
    no_time: bool,
}

fn main() {
    // Ask the user for input (skip if command line arguments are provided)
    let args = Args::parse();

    let n: usize = args.length;

    // time the calculation
    let (result, elapsed) = match args.single_threaded {
        false => par::fib(n),
        true => single::fib(n),
    };

    // Print the result
    match args.quiet {
        false => print_result(n, result, elapsed),
        true => (),
    }

    match args.no_time {
        false => println!("\tTime elapsed: {:?}", elapsed),
        true => (),
    }
}

fn print_result(n: usize, result: rug::Integer, elapsed: std::time::Duration) {
    println!("\n\tFibonacci sequence of length {}:", n);
    println!("\t{}", result);
    println!("\n\tLength in digits: {}", result.to_string().len());
    println!("\tTime elapsed: {:?}", elapsed);
}
