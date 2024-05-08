use std::time::Duration;

use rug::Integer;

pub fn fib(n: usize) -> (Integer, Duration) {
    let (a, b) = fib_u128(n);

    (Integer::from(a), b)
}

pub fn fib_u128(mut n: usize) -> (u128, Duration) {
    let mut f = [[0, 1], [1, 1]];
    let mut result = [[1, 0], [0, 1]];

    let time = std::time::Instant::now();
    while n > 0 {
        if n & 1 == 1 {
            // for the last calc we only need result[1][0] so we can skip the rest
            // (this is the most expensive part of the calculation)
            if n == 1 {
                return (
                    (result[1][0] * f[0][0]) + result[1][1] * f[1][0],
                    time.elapsed(),
                );
            }

            result = multiply(&result, &f);
        }
        f = multiply(&f, &f);
        n >>= 1;
    }

    let time = time.elapsed();

    (result[1][0], time)
}

pub fn multiply(a: &[[u128; 2]; 2], b: &[[u128; 2]; 2]) -> [[u128; 2]; 2] {
    let mut product = [[0, 0], [0, 0]];

    for i in 0..2 {
        for j in 0..2 {
            product[i][j] += (a[i][0] * b[0][j]) + (a[i][1] * b[1][j]);
        }
    }
    product
}
