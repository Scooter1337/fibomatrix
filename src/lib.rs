use wasm_bindgen::prelude::*;

// Import the existing Fibonacci implementations
// Note: You might need to make these modules public in their respective mod.rs files
// or reorganize your project structure for them to be accessible here.
// For now, let's assume they are accessible.
mod approx;
mod rayon;
mod simple;
mod threads;

#[wasm_bindgen]
pub fn fibonacci_iterative(n: usize) -> String {
    let (result, _elapsed) = simple::iter::fib(n);
    result.to_string()
}

#[wasm_bindgen]
pub fn fibonacci_matrix(n: usize) -> String {
    let (result, _elapsed) = simple::matrix::fib(n);
    result.to_string()
}

// Add more functions here for other methods as needed
// For example, for methods requiring precision:
// #[wasm_bindgen]
// pub fn fibonacci_approx(n: usize, precision: u32) -> String {
//     let (result, _elapsed) = approx::approx1::fib(n, precision);
//     result.to_string()
// }

// You'll also need to ensure that the modules (approx, rayon, simple, threads)
// and their submodules are correctly exposed.
// For example, in src/simple/mod.rs, you might have:
// pub mod iter;
// pub mod matrix;
// etc.

