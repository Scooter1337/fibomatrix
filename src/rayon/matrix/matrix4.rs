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
                let res2 = vec![0, 0];

                let res2 = res2
                    .par_iter()
                    .enumerate()
                    .map(|(i, _)| (&result[2 + i] * &f[i * 2]).complete())
                    .collect::<Vec<Integer>>();

                return ((&res2[0] + &res2[1]).complete(), time.elapsed());
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
    let products = vec![0, 0, 0, 0];

    let products = products
        .par_iter()
        .enumerate()
        .map(|(i, _)| {
            let indiv = vec![Integer::new(), Integer::new()];
            let x = i % 2;
            let y = i / 2;

            let indiv = indiv
                .par_iter()
                .enumerate()
                .map(|(j, _)| match j {
                    0 => (&a[(x * 2) + j] * &b[y]).complete(),
                    1 => (&a[(x * 2) + j] * &b[y + 2]).complete(),
                    _ => unreachable!(),
                })
                .collect::<Vec<Integer>>();

            (&indiv[0] + &indiv[1]).complete()
        })
        .collect::<Vec<Integer>>();

    products
}
