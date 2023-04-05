# WIP python library for r-like vectorized operations  
## This is only a proof of concept and for personal use, I don't indend on supporting this library.

This library is written entirely in rust and compiled to a python module using pyo3.

The main goal of this library is to provide a simple way to perform vectorized operations on lists of numbers, booleans, and strings. This library is inspired by the R language and the vectorized operations it provides. This library is not intended to be a replacement for numpy.  

For now, not all types support full cross-type operations. I will get around to adding support for all types eventually. I will probably create a python wrapper for the classes to take advantage of python's lack of type checking, and combine the classes into a single class.  

## Usage
```python
# The main types
from rvec import FloatVec, IntVec, BoolVec, StrVec

# general purpose concatenation function
from rvec import c

# from lists
x = FloatVec([1., 2., 3.])
y = IntVec([1, 2, 3])
z = BoolVec([True, False, True])
w = StrVec(['a', 'b', 'c'])

# concatanate any numbers or iterable of numbers
c(1, 2, 3)  # IntVec([1, 2, 3])
c(1, [2, 3])  # IntVec([1, 2, 3])
c((1, 2), (3, 4))  # IntVec([1, 2, 3, 4])
c(1, 2, 3.0)  # FloatVec([1., 2., 3.])
c(FloatVec([1., 2.]), IntVec([3, 4]))  # FloatVec([1., 2., 3., 4.])

# strings and booleans can only be concatenated with themselves or iterables of themselves
c('a', 'b', 'c')  # StrVec(['a', 'b', 'c'])
c(True, False, True)  # BoolVec([True, False, True])

# all math operations are elementwise
x + y  # FloatVec([2., 4., 6.])
x / 2  # FloatVec([0.5, 1., 1.5])

# all comparison operations are vectorized
x == y  # BoolVec([True, True, True])
x > y  # BoolVec([False, False, False])

# extends normal python slicing with vectorized indexing
x[x > 1]  # FloatVec([2., 3.])
x[IntVec([0, 2])]  # FloatVec([1., 3.])
x[1:3]  # FloatVec([2., 3.])
```