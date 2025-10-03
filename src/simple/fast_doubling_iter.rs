use rug::{Complete, Integer};
use std::time::{Duration, Instant};

// Iterative fast doubling avoids recursion by scanning bits of n from MSB to LSB.
pub fn fib(n: usize) -> (Integer, Duration) {
    if n == 0 {
        return (Integer::from(0), Duration::from_nanos(0));
    }
    if n < 3 {
        return (Integer::from(1), Duration::from_nanos(0));
    }

    let start = Instant::now();

    // Determine highest set bit (excluding leading zeros)
    let msb_index = usize::BITS as usize - (n.leading_zeros() as usize) - 1; // 0-based index

    // (a, b) will hold (F(k), F(k+1)) while processing
    let mut a = Integer::from(0); // F(0)
    let mut b = Integer::from(1); // F(1)

    // Process bits from msb_index down to 0
    for i in (0..=msb_index).rev() {
        // Loop invariant: currently have F(k)=a, F(k+1)=b for some k (starting with k=1?)
        // Doubling step: compute F(2k) and F(2k+1)
        // Using formulas with current a=F(k), b=F(k+1)
        let mut two_b_minus_a = b.clone();
        two_b_minus_a <<= 1; // 2*F(k+1)
        two_b_minus_a -= &a; // 2*F(k+1) - F(k)
        let c: Integer = (&a * &two_b_minus_a).complete(); // F(2k)
        let a_sq: Integer = (&a * &a).complete();
        let b_sq: Integer = (&b * &b).complete();
        let d: Integer = a_sq + b_sq; // F(2k+1)

        // Decide next based on bit i of n
        if (n >> i) & 1 == 0 {
            // even path: (F(2k), F(2k+1))
            a = c; // F(2k)
            b = d; // F(2k+1)
        } else {
            // odd path: (F(2k+1), F(2k)+F(2k+1)) = (d, c+d)
            a = d.clone();
            b = c + d;
        }
    }

    let elapsed = start.elapsed();
    (a, elapsed) // a = F(n)
}
