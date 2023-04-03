from rvec import c
import time
from operator import add, sub, mul, truediv, floordiv, mod

# test c function
print(c(1, 2, 3, 4, 5))
# c(1.0, 2.0, 3.0, 4.0, 5.0)
# c(True, False, True, False, True)
# c("a", "b", "c", "d", "e")

print(c([1, 2, 3, 4, 5]))
print(c((1, 2, 3, 4, 5)))

x = c(1, 2, 7, 15, 35)
y = c(5, 10, 15, 20, 25)
y_list = [5, 10, 15, 20, 25]
y_tuple = (5, 10, 15, 20, 25)

print("x: {}".format(x))
print("y: {}".format(y))

print("\ntesting +")
print("x + 2: {}".format(x + c(2)))
print("x + y: {}".format(x + y))

print("\ntesting -")
print("x - 2: {}".format(x - c(2)))
print("x - y: {}".format(x - y))

print("\ntesting *")
print("x * 2: {}".format(x * c(2)))
print("x * y: {}".format(x * y))

print("\ntesting /")
print("x / 2: {}".format(x / c(2)))
print("x / y: {}".format(x / y))

print("\ntesting //")
print("x // 2: {}".format(x // c(2)))
print("x // y: {}".format(x // y))

print("\ntesting %")
print("x % 2: {}".format(x % c(2)))
print("x % y: {}".format(x % y))

# print("testing **")

# benchmarking
print("\nbenchmarking")
x_list = [x for x in range(1, 10000000)]
y_list = [x for x in range(1, 10000000)]

x_tuple = tuple(x_list)
y_tuple = tuple(y_list)

from array import array

x_array = array('i', x_list)
y_array = array('i', y_list)

x = c(x_list)
y = c(y_list)

# time x + y
start = time.time()
x + y / c(2)
end = time.time()
print("x + y: {}".format(end - start))

# time x_list + y_list
start = time.time()
list(map(add, x_list, y_list))
end = time.time()
print("x_list + y_list: {}".format(end - start))

# time x_tuple + y_tuple
start = time.time()
tuple(map(add, x_tuple, y_tuple))
end = time.time()
print("x_tuple + y_tuple: {}".format(end - start))

# time x_array + y_array
start = time.time()
array('i', map(add, x_array, y_array))
end = time.time()
print("x_array + y_array: {}".format(end - start))
