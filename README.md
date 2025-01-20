# Fibo Matrix
Rust Command Line tool that calculates the n'th fibonacci number.

Created by [Luca](https://github.com/scooter1337)

# Bench
Method chosen is fastest for that N, that calculates the correct answer. (Approximations / Non-bigint-implementations do not always return the correct result, due to the nature of approximation and due to overflowing integers)
| N | time | method |
|---|---|---|
| 100 | 40ns | 4 |
| 1000 | 6.45us | 0 |
| 10K | 17.54us | 0 |
| 100K | 318us| 0 |
| 1M | 3.42ms| 8 |
| 10M | 33.73ms| 8 |
| 100M | 547ms | 8 |
| 1B | 5.82s | 8 |

# Usage
Build using `cargo build --release` or run `cargo run --release -- $` where $ is to be replaced with command-line arguments.

> Note
> 
> The first compile could take multiple minutes as 'gmp-mpfr-sys' is building. It is not frozen!

### Example
```bash
cargo build --release
target/release/fastest-fibo -m 8 --length 100000000 --quiet
```

or

```bash
cargo run --release -- -m 8 --length 100000000 --quiet --warmup 10
```


```bash
Usage: fibomatrix [OPTIONS] --length <LENGTH>

Options:
  -l, --length <LENGTH>
          Length of the Fibonacci sequence to generate
          
          This is the number of Fibonacci numbers to generate
          
          The minimum is 1

  -m, --method <METHOD>
          Method to use 
          
          
          Options: 
           
           Single-threaded: 
           > 0: Single-threaded matrix multiplication 
           > 1: Iterative 
           > 2: Recursive 
           > 3: Recursive with memoization 
           > 4: Single-threaded matrix multiplication without BigInt 
           
           Rayon (each variant is more optimized than the last): 
           > 5: Rayon matrix multiplication 1 
           > 6: Rayon matrix multiplication 2 
           > 7: Rayon matrix multiplication 3 
           > 8: Rayon matrix multiplication 4 
           
           (fastest) Threads: 
           > 9: Threads matrix multiplication 1 (WIP) 
           
           Approximation (BigFloat): 
           > 10: Approximation 1 
          
          
          [default: 1]

  -q, --quiet
          Don't print the result 
           (large numbers can take a long time to print, so this can save a lot of time)
          
          [default: false]

  -n, --no-time
          Don't print the time elapsed
          
          [default: false]

  -p, --precision <PRECISION>
          Precision for approximation
          
          [default: 100]

  -w, --warmup <WARMUP>
          Warmup iterations (number of times to run the calculation before timing it)
          
          [default: 0]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version

```
