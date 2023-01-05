/*
Baby-step giant-step approach
*/

use std::f64;
use std::collections::HashMap;
use num_bigint::{BigUint, BigInt};



fn bsgs_attempt(g : BigUint, p : BigUint, n : BigUint) {
    /* 
    Solve for x in h = g^x mod p given a prime p.
    If p is not prime, you shouldn't use BSGS anyway.
    */


    // Baby step
    let mut tbl = HashMap::new();
    let mut gi = BigUint::parse_bytes(b"1", 10).unwrap();


    let n_int: i32 = n.to_str_radix(10).parse().unwrap();
    
    for i in 0..n_int {
        tbl.insert(gi.to_str_radix(10), i);
        gi = &gi * &g % &p;
    }

    // Precompute via Fermat's Little Theorem
    let exp = &n * (&p - BigUint::from(2u32));
    let c = g.modpow(&exp, &p);

    println!("{:?}", tbl);
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

    let modulus = BigUint::parse_bytes(b"179769313486231590772930519078902473361797697", 10).unwrap();
    let x = BigUint::parse_bytes(b"578960446186580977117854925043439539266349923328202820", 10).unwrap();
    let y = BigUint::parse_bytes(b"76120582547389582323137850493559397925180747739176", 10).unwrap();
    let z = &x % &modulus;
    let w = &y % &modulus;
    let result = &z * &w % &modulus;



    let g = BigUint::parse_bytes(b"2", 10).unwrap();
    let q = BigUint::parse_bytes(b"10", 10).unwrap();
    let n = BigUint::parse_bytes(b"100", 10).unwrap();
    bsgs_attempt(g, q, n);

    println!("{:?}", result);

}
