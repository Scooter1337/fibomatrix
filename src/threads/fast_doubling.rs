use rug::{Complete, Integer};
use std::thread;
use std::time::{Duration, Instant};

// Spawn threads for the two square calculations at each recursion level.
fn fast_doubling(n: usize) -> (Integer, Integer) {
    // (F(n), F(n+1))
    if n == 0 {
        return (Integer::from(0), Integer::from(1));
    }
    if n == 1 {
        return (Integer::from(1), Integer::from(1));
    }

    let (fk, fk1) = fast_doubling(n >> 1);

    let mut two_fk1_minus_fk = fk1.clone();
    two_fk1_minus_fk <<= 1;
    two_fk1_minus_fk -= &fk;

    // Parallelize fk^2 and fk1^2 using threads (may be heavier than rayon for small sizes)
    let fk_clone = fk.clone();
    let fk1_clone = fk1.clone();
    let handle_a = thread::spawn(move || (&fk_clone * &fk_clone).complete());
    let handle_b = thread::spawn(move || (&fk1_clone * &fk1_clone).complete());

    let fk_sq = handle_a.join().unwrap();
    let fk1_sq = handle_b.join().unwrap();

    let c: Integer = (&fk * &two_fk1_minus_fk).complete();
    let d: Integer = fk_sq + fk1_sq;

    if n & 1 == 0 {
        (c, d)
    } else {
    (d.clone(), c + d)
    }
}

pub fn fib(n: usize) -> (Integer, Duration) {
    if n == 0 {
        return (Integer::from(0), Duration::from_nanos(0));
    }
    if n < 3 {
        return (Integer::from(1), Duration::from_nanos(0));
    }
    let start = Instant::now();
    let (f_n, _f_np1) = fast_doubling(n);
    let elapsed = start.elapsed();
    (f_n, elapsed)
}
