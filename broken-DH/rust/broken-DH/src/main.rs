/*
Baby-step giant-step approach
*/

use std::f64;


fn bsgs_attempt(cap : f64) -> f64 {
    /* 
    Solve for x in h = g^x mod p given a prime p.
    If p is not prime, you shouldn't use BSGS anyway.
    */

    let n = f64::ceil(f64::sqrt(cap))

    // Baby step
    let mut tbl = HashMap::new();
    let mut gi = 1;
    
    for i in 0..n {
        
    }
}



/*
fn bsgs_attempt(g, h, p, cap, debug=False):
    /* 
    Solve for x in h = g^x mod p given a prime p.
    If p is not prime, you shouldn't use BSGS anyway.
    */

    N = ceil(sqrt(cap))
    // Baby step.
    tbl = {}
    gi = 1
    for i in range(N):
        if debug & (i % 100000 == 0):
            print("We are at ith iteration: ", i)
        tbl[gi] = i
        gi = (gi * g) % p
    print("After hashmap") if debug else None

    // Precompute via Fermat's Little Theorem
    c = pow(g, N * (p - 2), p)

    cj = 1
    // Search for an equivalence in the table. Giant step.
    for j in range(N):
        
        if debug & (j % 100000 == 0):
            print("We are at jth iteration: ", j)
        
        cj = (cj * c) % p
        y = (h * cj) % p

        if y in tbl:
            sol = (j+1) * N + tbl[y]
            f = open("solution_r.txt", "a")
            f.write(str(sol))
            f.close()
            return sol

    // Solution not found
    return None
*/


fn main() {
    println!("Let us crack this challenge!\n");

    let n = 101.0;
    let (ceil_sqrt_n, sqrt_n) = bsgs_attempt(n);

    println!("The square root of {} is {} and its cealing is {}", n, sqrt_n, ceil_sqrt_n);


}
