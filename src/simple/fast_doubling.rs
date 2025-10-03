use rug::{Complete, Integer};
use std::time::{Duration, Instant};

// Fast doubling algorithm:
// F(2k) = F(k) * [2*F(k+1) − F(k)]
// F(2k+1) = F(k+1)^2 + F(k)^2
// Returns (F(n), F(n+1))
fn fib_fast_doubling(n: usize) -> (Integer, Integer) {
    if n == 0 {
        return (Integer::from(0), Integer::from(1));
    }
    if n == 1 {
        return (Integer::from(1), Integer::from(1));
    }

    // Recursive call
    let (fk, fk1) = fib_fast_doubling(n >> 1);

    // c = F(k) * (2*F(k+1) − F(k))
    // d = F(k)^2 + F(k+1)^2
    let mut two_fk1_minus_fk = fk1.clone();
    two_fk1_minus_fk <<= 1; // 2*F(k+1)
    two_fk1_minus_fk -= &fk; // 2*F(k+1) - F(k)
    let c: Integer = (&fk * &two_fk1_minus_fk).complete();
    let fk_sq: Integer = (&fk * &fk).complete();
    let fk1_sq: Integer = (&fk1 * &fk1).complete();
    let d: Integer = fk_sq + fk1_sq; // both complete()d

    if n & 1 == 0 {
        // even
        (c, d)
    } else {
        // odd
        // F(2k+1) = d; F(2k+2) = c + d
        (d.clone(), c + d)
    }
}

pub fn fib(n: usize) -> (Integer, Duration) {
    if n == 0 {
        // For consistency with other code expecting length >=1, but handle gracefully
        return (Integer::from(0), Duration::from_nanos(0));
    }
    if n < 3 {
        // F(1)=1, F(2)=1
        return (Integer::from(1), Duration::from_nanos(0));
    }
    let start = Instant::now();
    let (f_n, _f_n1) = fib_fast_doubling(n);
    let elapsed = start.elapsed();
    (f_n, elapsed)
}
