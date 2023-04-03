from rvec import RInt#, RFloat, RBool, RString

def c(*args):
    # get the type of the first argument, if it is a list, get the type of the first element
    if isinstance(args[0], list):
        first = args[0][0]
    else:
        first = args[0]

    if isinstance(first, int):
        out = RInt.new()
    elif isinstance(first, float):
        out = RFloat.new()
    elif isinstance(first, bool):
        out = RBool.new()
    elif isinstance(first, str):
        out = RString.new()
    else:
        raise TypeError("Unsupported type: {}".format(type(first)))

    for arg in args:
        if isinstance(arg, list):
            out.push_list(arg)
        if isinstance(arg, type(out)):
            out.concat(arg)
        else:
            out.push(arg)

    return out

x = c(1, 2, 3, [4, 5, 6, 7, 8, 9, 10])
print(x)

# y = c(1, 2, 3, [4, 5, 6, 7, 8, 9, 10], x)
# print(y)

# print(y[0])

# y[0] = 100
# print(y)