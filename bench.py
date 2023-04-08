from rvec import RVec
import time
from operator import add, sub, mul, truediv, floordiv, mod
from random import random

# benchmarking
print("\nbenchmarking")
x_list = [random() for _ in range(1000000)]
y_list = [random() for _ in range(1000000)]
z_list = [random() for _ in range(1000000)]

# start = time.time()
# x = c(x_list)
# y = c(y_list)
# z = c(z_list)
# end = time.time()
# c_alloc_time = end - start
# print("alloc c: {}".format(c_alloc_time))

start = time.time()
x = RVec(x_list)
y = RVec(y_list)
z = RVec(z_list)
end = time.time()
fv_alloc_time = end - start
print("alloc RVec: {}".format(fv_alloc_time))

def func(x, y, z):
    return x * 2 + y / (z + 5)

start = time.time()
func(x, y, z)
end = time.time()
print("\nx + y: {}".format(end - start))
rvec_time = end - start

start = time.time()
list(map(func, x_list, y_list, z_list))
end = time.time()
print("x_list + y_list: {}".format(end - start))
list_time = end - start

print("\nrvec is {} times the speed of lists".format(list_time / rvec_time))
# print("with c alloc time: {}".format(list_time / (rvec_time + c_alloc_time)))
print("with RVec alloc time: {}".format(list_time / (rvec_time + fv_alloc_time)))
