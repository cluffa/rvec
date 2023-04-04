from rvec import c
import time
from operator import add, sub, mul, truediv, floordiv, mod
from random import randint

# benchmarking
print("\nbenchmarking")
x_list = [randint(-100, 100) for _ in range(1000000)]
y_list = [randint(-100, 100) for _ in range(1000000)]
z_list = [randint(1, 100) for _ in range(1000000)]

x = c(x_list)
y = c(y_list)
z = c(z_list)

def func(x, y, z):
    return x + y / z * 2

start = time.time()
func(x, y, z)
end = time.time()
print("x + y: {}".format(end - start))
rvec_time = end - start

start = time.time()
list(map(func, x_list, y_list, z_list))
end = time.time()
print("x_list + y_list: {}".format(end - start))
list_time = end - start

print("\nrvec is {} times faster than list".format(list_time / rvec_time))
