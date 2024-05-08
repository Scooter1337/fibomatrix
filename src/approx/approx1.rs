#![allow(unused_assignments)]
use std::{
    ops::{Div, Mul},
    time::Duration,
};

use rug::{ops::Pow, Float, Integer};

pub fn fib(n: usize, precision: u32) -> (Integer, Duration) {
    let now = std::time::Instant::now();
    let mut sqrt_of_5 = Float::new(precision);
    sqrt_of_5 += 5;
    sqrt_of_5 = sqrt_of_5.sqrt();

    let mut e = Float::new(precision);
    e += 1;
    e = e.div(sqrt_of_5.clone());

    let mut one_minus_sqrt5 = Float::new(precision);
    one_minus_sqrt5 += 1;
    one_minus_sqrt5 -= sqrt_of_5.clone() + 1;

    let mut one_plus_sqrt5 = Float::new(precision);
    one_plus_sqrt5 += 1;
    one_plus_sqrt5 += sqrt_of_5.clone();

    let mut a = Float::new(precision);
    a = one_plus_sqrt5.clone().div(2);
    a = a.pow(n);

    let mut b = Float::new(precision);
    b = one_minus_sqrt5.clone().div(2);
    b = b.pow(n);

    let mut res = Float::new(precision);
    res = e.clone().mul(a.clone()) - e.clone().mul(b.clone());

    let duration = now.elapsed();

    let res = res.round().to_integer();

    match res {
        Some(res) => (res, duration),
        None => panic!("Failed to convert Float to Integer, might be too large"),
    }
}
