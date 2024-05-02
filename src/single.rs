use std::time::Duration;

use rug::{Complete, Integer};

pub fn fib(mut n: usize) -> (Integer, Duration) {
    let mut f = [
        [Integer::new(), Integer::from(1)],
        [Integer::from(1), Integer::from(1)],
    ];
    let mut result = [
        [Integer::from(1), Integer::new()],
        [Integer::new(), Integer::from(1)],
    ];

    let time = std::time::Instant::now();
    while n > 0 {
        if n & 1 == 1 {
            result = multiply(&result, &f);
        }
        f = multiply(&f, &f);
        n >>= 1;
    }

    let time = time.elapsed();

    (result[1][0].clone(), time)
}

pub fn multiply(a: &[[Integer; 2]; 2], b: &[[Integer; 2]; 2]) -> [[Integer; 2]; 2] {
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
