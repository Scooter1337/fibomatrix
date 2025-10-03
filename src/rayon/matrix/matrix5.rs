use rayon::join;
use rug::{Complete, Integer};
use std::time::{Duration, Instant};

// Optimized parallel matrix exponentiation for Fibonacci using a fixed-size 2x2 matrix
// Representation: [a00, a01, a10, a11]
// We avoid Vec allocations inside the loop and reuse Integer objects by overwriting them.
// Parallelism: we parallelize the 4 products of a 2x2 * 2x2 multiply using two join levels.
// For very large n, big-int multiplication dominates; reducing allocation / cloning helps.
// This version performs fewer temporary allocations than matrix4 and avoids building Vecs per multiply.

#[inline]
fn mul2x2(a: &[Integer; 4], b: &[Integer; 4], out: &mut [Integer; 4]) {
    // Compute c = a*b
    // c00 = a00*b00 + a01*b10
    // c01 = a00*b01 + a01*b11
    // c10 = a10*b00 + a11*b10
    // c11 = a10*b01 + a11*b11
    // We structure as two parallel halves (top row, bottom row), and inside each row two joins for the pair.

    let (a00, a01, a10, a11) = (&a[0], &a[1], &a[2], &a[3]);
    let (b00, b01, b10, b11) = (&b[0], &b[1], &b[2], &b[3]);

    // Top row join
    let (c00, c01) = join(
        || (a00 * b00).complete() + (a01 * b10),
        || (a00 * b01).complete() + (a01 * b11),
    );

    // Bottom row join
    let (c10, c11) = join(
        || (a10 * b00).complete() + (a11 * b10),
        || (a10 * b01).complete() + (a11 * b11),
    );

    out[0] = c00; // a00
    out[1] = c01; // a01
    out[2] = c10; // a10
    out[3] = c11; // a11
}

pub fn fib(mut n: usize) -> (Integer, Duration) {
    if n == 0 {
        return (Integer::from(0), Duration::from_nanos(0));
    }
    if n < 3 {
        // F(1)=1, F(2)=1
        return (Integer::from(1), Duration::from_nanos(0));
    }

    // f is the base matrix [[0,1],[1,1]]
    let mut f = [
        Integer::from(0),
        Integer::from(1),
        Integer::from(1),
        Integer::from(1),
    ];
    // result is identity [[1,0],[0,1]]
    let mut result = [
        Integer::from(1),
        Integer::from(0),
        Integer::from(0),
        Integer::from(1),
    ];

    // Temporary buffers to avoid reallocations
    let mut tmp1 = [
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
    ];
    let mut tmp2 = [
        Integer::new(),
        Integer::new(),
        Integer::new(),
        Integer::new(),
    ];

    let start = Instant::now();
    while n > 0 {
        if n & 1 == 1 {
            if n == 1 {
                // Only need result[2] (row 1 col 0) after multiplying result * f
                // Compute that cell directly: res10 = result10*f00 + result11*f10
                let res10: Integer = (&result[2] * &f[0]).complete() + (&result[3] * &f[2]);
                return (res10, start.elapsed());
            }
            mul2x2(&result, &f, &mut tmp1);
            // swap tmp1 into result (move Integers)
            result.swap_with_slice(&mut tmp1);
        }
        // f = f * f (square)
        mul2x2(&f, &f, &mut tmp2);
        f.swap_with_slice(&mut tmp2);
        n >>= 1;
    }

    let elapsed = start.elapsed();
    (result[2].clone(), elapsed)
}
