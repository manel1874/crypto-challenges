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

### Find r

As 



