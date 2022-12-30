#include <iostream>
#include <cmath>
#include <map>
using namespace std;


float bsgs_attempt(int g, int h, int p, int cap, bool debug){

    /*
    Solve for x in h = g^x mod p given a prime p.
    If p is not prime, you shouldn't use BSGS anyway.
    */
    
    float N = ceil(sqrt(cap));

    map<int, int> tbl = {};



    return N;
    
    /*
    # Baby step.
    tbl = {}
    gi = 1
    for i in range(N):
        if debug & (i % 100000 == 0):
            print("We are at ith iteration: ", i)
        tbl[gi] = i
        gi = (gi * g) % p
    print("After hashmap") if debug else None

    # Precompute via Fermat's Little Theorem
    c = pow(g, N * (p - 2), p)

    cj = 1
    # Search for an equivalence in the table. Giant step.
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

    # Solution not found
    return None

    */

}




int main() {
    std::cout << "|== c++ implementation ==|\n";

    float N = bsgs_attempt(12, 13, 2, 6, true);

    std::cout << N << "\n";

    return 0;
}