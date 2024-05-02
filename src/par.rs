use std::time::Duration;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rug::{Complete, Integer};

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
            result = multiply(&result, &f);
        }
        f = multiply(&f, &f);
        n >>= 1;
    }

    let time = time.elapsed();

    (result[1][0].clone(), time)
}

fn multiply(a: &Vec<Vec<Integer>>, b: &Vec<Vec<Integer>>) -> Vec<Vec<Integer>> {
    let product = [
        [Integer::new(), Integer::new()],
        [Integer::new(), Integer::new()],
    ];

    let product = product
        .par_iter()
        .enumerate()
        .map(|(x, i)| {
            return i
                .par_iter()
                .enumerate()
                .map(|(y, _j)| (&a[x][0] * &b[0][y]).complete() + (&a[x][1] * &b[1][y]))
                .collect::<Vec<Integer>>();
        })
        .collect::<Vec<Vec<Integer>>>();

    product
}
