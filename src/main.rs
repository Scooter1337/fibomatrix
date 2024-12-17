#![allow(clippy::needless_range_loop)]

use std::io::Write;

use clap::Parser;
mod approx;
mod rayon;
mod simple;
mod threads;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Length of the Fibonacci sequence to generate
    ///
    /// This is the number of Fibonacci numbers to generate
    ///
    /// The minimum is 1
    #[clap(short, long)]
    length: usize,

    /// Method to use {n}
    ///
    /// Options: {n} {n}
    /// Single-threaded: {n}
    /// > 0: Single-threaded matrix multiplication {n}
    /// > 1: Iterative {n}
    /// > 2: Recursive {n}
    /// > 3: Recursive with memoization {n}
    /// > 4: Single-threaded matrix multiplication without BigInt {n} {n}
    /// Rayon (each variant is more optimized than the last): {n}
    /// > 5: Rayon matrix multiplication 1 {n}
    /// > 6: Rayon matrix multiplication 2 {n}
    /// > 7: Rayon matrix multiplication 3 {n}
    /// > 8: Rayon matrix multiplication 4 {n} {n} (fastest)
    /// Threads: {n}
    /// > 9: Threads matrix multiplication 1 (WIP) {n} {n}
    /// Approximation (BigFloat): {n}
    /// > 10: Approximation 1 {n}
    #[clap(short, long, default_value = "1")]
    method: usize,

    /// Don't print the result {n}
    /// (large numbers can take a long time to print, so this can save a lot of time)
    ///
    /// [default: false]
    #[clap(long, short)]
    quiet: bool,

    /// Don't print the time elapsed
    ///
    /// [default: false]
    #[clap(long, short)]
    no_time: bool,

    /// Precision for approximation
    ///
    #[clap(long, short, default_value = "100")]
    precision: u32,

    /// Warmup iterations
    /// (number of times to run the calculation before timing it)
    ///
    #[clap(long, short, default_value = "0")]
    warmup: usize,
}

fn main() {
    // Ask the user for input (skip if command line arguments are provided)
    let args = Args::parse();

    let n: usize = args.length;

    if n < 1 {
        println!("Length must be at least 1");
        return;
    }

    // Warmup
    if args.warmup > 0 {
        println!("Warming up...");
        for i in 0..args.warmup {
            match args.method {
                0 => simple::matrix::fib(n),
                1 => simple::iter::fib(n),
                2 => simple::recursive::fib(n),
                3 => simple::recursive_memo::fib(n),
                4 => simple::matrix_no_bigint::fib(n),
                5 => rayon::matrix1::fib(n),
                6 => rayon::matrix2::fib(n),
                7 => rayon::matrix3::fib(n),
                8 => rayon::matrix4::fib(n),
                9 => threads::matrix1::fib(n),
                10 => approx::approx1::fib(n, args.precision),
                _ => unimplemented!("Method not implemented"),
            };
            print!("\rWarmup: {}/{}", i + 1, args.warmup);
            std::io::stdout().flush().unwrap();
        }
        println!("\nWarmup complete\n");
    }

    // time the calculation
    let (result, elapsed) = match args.method {
        0 => simple::matrix::fib(n),
        1 => simple::iter::fib(n),
        2 => simple::recursive::fib(n),
        3 => simple::recursive_memo::fib(n),
        4 => simple::matrix_no_bigint::fib(n),
        5 => rayon::matrix1::fib(n),
        6 => rayon::matrix2::fib(n),
        7 => rayon::matrix3::fib(n),
        8 => rayon::matrix4::fib(n),
        9 => threads::matrix1::fib(n),
        10 => approx::approx1::fib(n, args.precision),
        _ => unimplemented!("Method not implemented"),
    };

    // Print the result
    match args.quiet {
        false => print_result(n, result),
        true => (),
    }

    match args.no_time {
        false => println!("\tTime elapsed: {:?}", elapsed),
        true => (),
    }
}

fn print_result(n: usize, result: rug::Integer) {
    let string = result.to_string();
    let length = string.len();

    println!("\n\tFibonacci sequence of length {}:", n);
    println!("\t{}", &string);
    println!("\n\tLength in digits: {}", length);
}
