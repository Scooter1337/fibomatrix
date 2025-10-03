// Simplified benchmark binary for fast doubling variants using the library crate.
fn main() {
    let test_ns = [1000usize, 5000, 10_000, 25_000, 50_000];
    println!("n,fd_recursive_ns,fd_iter_ns,fd_rayon_ns,fd_threads_ns");
    for &n in &test_ns {
        let (_v, t11) = fastest_fibo::simple::fast_doubling::fib(n);
        let (_v, t12) = fastest_fibo::simple::fast_doubling_iter::fib(n);
        let (_v, t13) = fastest_fibo::rayon::fast_doubling::fib(n);
        let (_v, t14) = fastest_fibo::threads::fast_doubling::fib(n);
        println!("{},{:?},{:?},{:?},{:?}", n, t11.as_nanos(), t12.as_nanos(), t13.as_nanos(), t14.as_nanos());
    }
}
