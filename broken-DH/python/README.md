# Python solution


### Check Bob's final step (4)

First, let us check Bob's final step (4):


The naive approach in Python is to try the following:

```
proved = _g**_t % _p == _a * _h**_c %_p

print("Bob can check Alice's knowledge: ", proved)
```

However, due to the dimension of the numbers, we cannot do it in this way. We can, but probably that is the only this that we will 
be able to do during our lifetime...

The issue here comes from the fact that the modulo operation (%) is only carried out after computing the exponentiation (e.g. _g**_t). 

We need a more efficient way to multiply and exponentiate big numbers. This is achieved with the `pow()` operation. Indeed, we can run:

```
proved = pow(_g, _t, _p) == _a * pow(_h, _c, _p) % _p

print("Bob can check Alice's knowledge: ", proved)
```

By running the script we get:

```
>>> Bob can check Alice's knowledge:  True
>>>
>>> real    0m0.068s
>>> user    0m0.060s
>>> sys     0m0.004s
```

## First approach (brute-force)

The aim of this brute force approach is simply to run through all possible values `r` given both `_a` and `_g`. As stated in the main article this amounts for at most `2^50 = 1 125 899 906 842 624` tests. 

### First attempt

In our first attempt, we use the function `bf_1st_attempt`. Here, in every iteration we compute `pow(_g, r, _p)` for different values of `r`. If we follow this approach for `my_range = 2**15`, we get:

```
>>> None
>>> Total time in the first attempt: 8.1058
```

For `my_range = 2**50`, and assuming the time is linear with `my_range` (which is not the case), it would take around `8 * 2**35 = 274877906944` seconds (around 8 716 years...) to find the desired `r`. Not good enough...

### Second attempt

We can improve the computation efficiency inside the `for` loop by avoiding using the `pow` operation. In fact, for every iteration we are computing a new exponential that only increases by one with respect to the previous iteration. 

In this second attempt, we avoid using the `pow` operation and only use modular multiplication. If we follow this approach for `my_range = 2**15`, we get:

```
>>> None
>>> Total time in the first attempt: 0.4816
```

It is a little better but definitly not enough. It would take around 523 years to go through all cases. We have leave this naive brute force approach and try a new one.


## Second approach (baby-step giant-step)

We can find the solution using a tailored version of baby-step giant-step. This tailored version takes into consideration that `r` has at most 50-bit length. This is achieved by function `bsgs_attempt()` which saves the solution to the file `solution_r.txt`. It took us 21m51s to find the solution! Yay! Much faster than 523 years...!

The final solution `x` was found using the function `get_x()`, which saves the solution to the file `solution_x.txt`. To compute with big numbers we used the `decimal` library, which allows to define the precision required for each use case. 


## Important libraries

- Big integers library: `decimal`


## References

[1] [Asecurity site - Baby-Step-Giant-Step](https://asecuritysite.com/encryption/baby)







