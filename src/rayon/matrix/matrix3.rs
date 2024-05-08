use std::time::Duration;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rug::{Complete, Integer};

pub fn fib(mut n: usize) -> (Integer, Duration) {
    let mut f = vec![
        Integer::new(),
        Integer::from(1),
        Integer::from(1),
        Integer::from(1),
    ];
    let mut result = vec![
        Integer::from(1),
        Integer::new(),
        Integer::new(),
        Integer::from(1),
    ];

    let time = std::time::Instant::now();
    while n > 0 {
        if n & 1 == 1 {
            // for the last calc we only need result[1][0] so we can skip the rest
            // (this is the most expensive part of the calculation)
            if n == 1 {
                result[2] = (&result[2] * &f[0]).complete() + &result[3] * &f[2];

                break;
            }

            result = mult(&result, &f);
        }

        f = mult(&f, &f);
        n >>= 1;
    }

    let time = time.elapsed();

    (result[2].to_owned(), time)
}

fn mult(a: &[Integer], b: &[Integer]) -> Vec<Integer> {
    let products = vec![
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
    ];

    let products = products
        .par_iter()
        .enumerate()
        .map(|(i, _)| {
            let x = i % 4;
            let y = i / 4;

            match i & 1 {
                0 => (&a[x] * &b[y]).complete(),
                1 => (&a[x] * &b[2 + y]).complete(),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Integer>>();

    let additions = vec![
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
    ];

    let additions = additions
        .par_iter()
        .enumerate()
        .map(|(i, _)| (&products[i * 2] + &products[(i * 2) + 1]).complete())
        .collect::<Vec<Integer>>();

    additions
}
