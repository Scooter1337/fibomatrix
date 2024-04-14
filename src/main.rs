#![allow(clippy::needless_range_loop)]
use rug::{Complete, Integer};
use std::io;

fn main() {
    // Ask the user for input (skip if command line arguments are provided)
    let mut n = String::new();
    match std::env::args().nth(1) {
        Some(arg) => n = arg,
        None => {
            println!("Enter the position of the Fibonacci number you want to calculate:");
            io::stdin().read_line(&mut n).expect("Failed to read line");
        }
    };

    let n: usize = n.trim().parse().expect("Please type a number!");

    // time the calculation
    let time = std::time::Instant::now();
    let result = fib(n);
    let elapsed = time.elapsed();

    // Print the result
    println!("Fibonacci number at position {}: \n{}", n, result);
    println!("\n\tLength in digits: {}", result.to_string().len());
    println!("\tTime elapsed: {:?}", elapsed);
}

fn fib(mut n: usize) -> Integer {
    let mut f = [
        [Integer::new(), Integer::from(1)],
        [Integer::from(1), Integer::from(1)],
    ];
    let mut result = [
        [Integer::from(1), Integer::new()],
        [Integer::new(), Integer::from(1)],
    ];

    while n > 0 {
        if n & 1 == 1 {
            result = multiply(&result, &f);
        }
        f = multiply(&f, &f);
        n >>= 1;
    }

    result[1][0].clone()
}

fn multiply(a: &[[Integer; 2]; 2], b: &[[Integer; 2]; 2]) -> [[Integer; 2]; 2] {
    let mut product = [
        [Integer::new(), Integer::new()],
        [Integer::new(), Integer::new()],
    ];

    for i in 0..2 {
        for j in 0..2 {
            product[i][j] += (&a[i][0] * &b[0][j]).complete() + (&a[i][1] * &b[1][j]);
        }
    }
    product
}
