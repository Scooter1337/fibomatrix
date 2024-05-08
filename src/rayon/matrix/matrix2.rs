use super::multiply1::multiply;
use rug::{Complete, Integer};
use std::time::Duration;

pub fn fib(mut n: usize) -> (Integer, Duration) {
    let mut f = vec![
        vec![Integer::new(), Integer::from(1)],
        vec![Integer::from(1), Integer::from(1)],
    ];
    let mut result = vec![
        vec![Integer::from(1), Integer::new()],
        vec![Integer::new(), Integer::from(1)],
    ];

    let time = std::time::Instant::now();
    while n > 0 {
        if n & 1 == 1 {
            // for the last calc we only need result[1][0] so we can skip the rest
            // (this is the most expensive part of the calculation)
            if n == 1 {
                return (
                    (&result[1][0] * &f[0][0]).complete() + &result[1][1] * &f[1][0],
                    time.elapsed(),
                );
            }

            result = multiply(&result, &f);
        }

        f = multiply(&f, &f);
        n >>= 1;
    }

    let time = time.elapsed();

    (result[1][0].clone(), time)
}
