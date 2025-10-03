use rayon::join;
use rug::{Complete, Integer};
use std::time::{Duration, Instant};

// Parallel fast doubling: parallelize the two square multiplications per level.
fn fast_doubling(n: usize) -> (Integer, Integer) {
    // (F(n), F(n+1))
    if n == 0 {
        return (Integer::from(0), Integer::from(1));
    }
    if n == 1 {
        return (Integer::from(1), Integer::from(1));
    }

    let (fk, fk1) = fast_doubling(n >> 1);

    // Compute: c = F(k)*(2*F(k+1)-F(k)); d = F(k)^2 + F(k+1)^2
    // Parallelize fk^2 and fk1^2 multiplications.
    let mut two_fk1_minus_fk = fk1.clone();
    two_fk1_minus_fk <<= 1;
    two_fk1_minus_fk -= &fk;

    let (fk_sq, fk1_sq) = join(|| (&fk * &fk).complete(), || (&fk1 * &fk1).complete());

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
