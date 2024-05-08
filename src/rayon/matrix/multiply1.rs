#![allow(clippy::ptr_arg)]
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rug::{Complete, Integer};

pub fn multiply(a: &Vec<Vec<Integer>>, b: &Vec<Vec<Integer>>) -> Vec<Vec<Integer>> {
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
