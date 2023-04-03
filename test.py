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
x_list = [x for x in range(1, 10000000)]
y_list = [x for x in range(1, 10000000)]

x_tuple = tuple(x_list)
y_tuple = tuple(y_list)

x = c(x_list)
y = c(y_list)

# time x + y
start = time.time()
x + y
end = time.time()
print("x + y: {}".format(end - start))

# time x_list + y_list
start = time.time()
map(add, x_list, y_list)
end = time.time()
print("x_list + y_list: {}".format(end - start))

# time x_tuple + y_tuple
start = time.time()
map(add, x_tuple, y_tuple)
end = time.time()
print("x_tuple + y_tuple: {}".format(end - start))


# time x - y
start = time.time()
x - y
end = time.time()
print("x - y: {}".format(end - start))

# time x_list - y_list
start = time.time()
map(sub, x_list, y_list)
end = time.time()
print("x_list - y_list: {}".format(end - start))

# time x_tuple - y_tuple
start = time.time()
map(sub, x_tuple, y_tuple)
end = time.time()
print("x_tuple - y_tuple: {}".format(end - start))

# time x * y
start = time.time()
x * y
end = time.time()
print("x * y: {}".format(end - start))

# time x_list * y_list
start = time.time()
map(mul, x_list, y_list)
end = time.time()
print("x_list * y_list: {}".format(end - start))

# time x_tuple * y_tuple
start = time.time()
map(mul, x_tuple, y_tuple)
end = time.time()
print("x_tuple * y_tuple: {}".format(end - start))

# time x / y
start = time.time()
x / y
end = time.time()
print("x / y: {}".format(end - start))

# time x_list / y_list
start = time.time()
map(truediv, x_list, y_list)
end = time.time()
print("x_list / y_list: {}".format(end - start))

# time x_tuple / y_tuple
start = time.time()
map(truediv, x_tuple, y_tuple)
end = time.time()
print("x_tuple / y_tuple: {}".format(end - start))

# time x // y
start = time.time()
x // y
end = time.time()
print("x // y: {}".format(end - start))

# time x_list // y_list
start = time.time()
map(floordiv, x_list, y_list)
end = time.time()
print("x_list // y_list: {}".format(end - start))