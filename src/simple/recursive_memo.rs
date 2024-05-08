pub fn fib(n: usize) -> (rug::Integer, std::time::Duration) {
    let now = std::time::Instant::now();

    let result = fib_recursive_memo(n, &mut vec![rug::Integer::new(); n + 1]);
    let duration = now.elapsed();
    (result, duration)
}

fn fib_recursive_memo(n: usize, memo: &mut Vec<rug::Integer>) -> rug::Integer {
    if n < 3 {
        memo[n] = rug::Integer::from(1);
    } else if memo[n].is_zero() {
        memo[n] = fib_recursive_memo(n - 1, memo) + fib_recursive_memo(n - 2, memo);
    }
    memo[n].clone()
}
