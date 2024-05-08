use std::time::Duration;

use rug::Integer;

pub fn fib(n: usize) -> (Integer, Duration) {
    let now = std::time::Instant::now();
    let result = fib_recursive(n);
    let duration = now.elapsed();
    (result, duration)
}

fn fib_recursive(n: usize) -> Integer {
    if n < 3 {
        Integer::from(1)
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}
