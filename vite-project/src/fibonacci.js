import BN from 'bn.js';

export function fib(n) {
    let f = [
        new BN(0),
        new BN(1),
        new BN(1),
        new BN(1),
    ];
    let result = [
        new BN(1),
        new BN(0),
        new BN(0),
        new BN(1),
    ];

    const startTime = performance.now();
    while (n > 0) {
        if (n % 2 === 1) {
            if (n === 1) {
                const res2_0 = result[2].mul(f[0]).add(result[3].mul(f[1]));
                // const res2_1 = result[2].mul(f[2]).add(result[3].mul(f[3])); // Not strictly needed for the final result based on the Rust code's optimization
                const finalResult = res2_0; // Simplified as per Rust's optimization for the last step
                const endTime = performance.now();
                return { value: finalResult, duration: endTime - startTime };
            }
            result = mult(result, f);
        }

        f = mult(f, f);
        n = Math.floor(n / 2);
    }

    const endTime = performance.now();
    // The Rust code returns result[2] which corresponds to F(n) in the matrix [F(n+1), F(n); F(n), F(n-1)]
    // After the loop, if n was even and became 0, result holds the identity matrix or a power of the Fibonacci matrix.
    // If the original n was 0, this part might need adjustment based on expected F(0).
    // Assuming n > 0 as per the Rust code's logic, result[2] should be F(n).
    // If n was initially 0, the loop doesn't run, and result[2] is 0n.
    // The Rust code seems to handle n=0 by returning result[2] which is initialized to 0.
    // However, the problem usually defines fib(0)=0, fib(1)=1.
    // The matrix [1,0;0,1] * [0,1;1,1]^n should yield [F(n-1), F(n); F(n), F(n+1)] if result starts as identity.
    // The provided Rust code initializes result = [1,0,0,1] and f = [0,1,1,1] (transposed Q matrix essentially)
    // Let's trace for n=1:
    // loop 1: n=1. n&1==1. n==1.
    //   res2_0 = result[2]*f[0] + result[3]*f[1] = 0n*0n + 1n*1n = 1n.
    //   returns (1n, time) -> Correct for F(1)
    // Let's trace for n=2:
    // loop 1: n=2. n&1==0.
    //   f = mult([0,1,1,1], [0,1,1,1])
    //     f[0] = 0*0+1*1 = 1
    //     f[1] = 0*1+1*1 = 1
    //     f[2] = 1*0+1*1 = 1
    //     f[3] = 1*1+1*1 = 2
    //   f becomes [1,1,1,2]
    //   n = 1
    // loop 2: n=1. n&1==1. n==1.
    //   res2_0 = result[2]*f[0] + result[3]*f[1] = 0n*1n + 1n*1n = 1n
    //   returns (1n, time) -> Correct for F(2)
    // Let's trace for n=3:
    // loop 1: n=3. n&1==1. n!=1.
    //   result = mult([1,0,0,1], [0,1,1,1])
    //     result[0] = 1*0+0*1 = 0
    //     result[1] = 1*1+0*1 = 1
    //     result[2] = 0*0+1*1 = 1
    //     result[3] = 0*1+1*1 = 1
    //   result becomes [0,1,1,1]
    //   f = mult([0,1,1,1], [0,1,1,1]) -> [1,1,1,2]
    //   n = 1
    // loop 2: n=1. n&1==1. n==1.
    //   res2_0 = result[2]*f[0] + result[3]*f[1] = 1n*1n + 1n*1n = 2n
    //   returns (2n, time) -> Correct for F(3)

    // The final result in the Rust code is `result[2]`.
    return { value: result[2], duration: endTime - startTime };
}

function mult(a, b) {
    // Matrix A = [a0 a1]   Matrix B = [b0 b1]
    //            [a2 a3]              [b2 b3]
    // Result C = [c0 c1]
    //            [c2 c3]
    // c0 = a0*b0 + a1*b2
    // c1 = a0*b1 + a1*b3
    // c2 = a2*b0 + a3*b2
    // c3 = a2*b1 + a3*b3

    // The Rust code's mult function seems to implement a specific matrix multiplication
    // where the matrices are stored row by row in a flat array: [row1_col1, row1_col2, row2_col1, row2_col2]
    // Let a = [a[0], a[1], a[2], a[3]] and b = [b[0], b[1], b[2], b[3]]
    // The mapping in the Rust code's `mult` function:
    // products[i] where i ranges from 0 to 3.
    // x = (i % 2) * 2;  // 0 for i=0,1; 2 for i=2,3
    // y = i / 2;        // 0 for i=0,2; 1 for i=1,3 (integer division)

    // For i = 0: x=0, y=0. product = a[0]*b[0] + a[1]*b[2]  (This is c0)
    // For i = 1: x=0, y=0. product = a[0]*b[1] + a[1]*b[3]  (This is c1)
    // For i = 2: x=2, y=1. product = a[2]*b[0] + a[3]*b[2]  (This is c2)
    // For i = 3: x=2, y=1. product = a[2]*b[1] + a[3]*b[3]  (This is c3)
    // This matches standard matrix multiplication.

    const c0 = a[0].mul(b[0]).add(a[1].mul(b[2]));
    const c1 = a[0].mul(b[1]).add(a[1].mul(b[3]));
    const c2 = a[2].mul(b[0]).add(a[3].mul(b[2]));
    const c3 = a[2].mul(b[1]).add(a[3].mul(b[3]));

    return [c0, c1, c2, c3];
}
