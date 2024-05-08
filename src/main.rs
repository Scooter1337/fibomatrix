#![allow(clippy::needless_range_loop)]

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
    /// Options: {n}
    /// > 0: Single-threaded {n}
    /// > 1: Iterative {n}
    /// > 2: Recursive {n}
    /// > 3: Recursive with memoization {n}
    ///
    /// > 4: Parallel (rayon) {n}
    /// > 5: Parallel (rayon) but skips 3/4 of the last iteration {n}
    /// > 6: Parallel (rayon) but also parallelizes the multiplications, instead of just the 4 matrix values {n}
    /// > 7: Combination of 2 and 3, which doesn't have to wait for all multiplications to finish {n}
    ///
    /// > 8: Parallel (threads) {n}
    ///
    /// > 9: Approximation (Big Float, with chosen precision) {n}
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
}

fn main() {
    // Ask the user for input (skip if command line arguments are provided)
    let args = Args::parse();

    let n: usize = args.length;

    if n < 1 {
        println!("Length must be at least 1");
        return;
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
