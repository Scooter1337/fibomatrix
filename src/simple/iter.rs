use std::time::Duration;

use rug::{Complete, Integer};

pub fn fib(n: usize) -> (Integer, Duration) {
    let mut a = Integer::from(1);
    let mut b = Integer::from(1);

    let now = std::time::Instant::now();

    for _ in 3..=n {
        let c = (&a + &b).complete();
        a = b;
        b = c.clone();
    }

    let duration = now.elapsed();

    (b, duration)
}
