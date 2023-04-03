from .rvec import *

print("running rvec/__init__.py")

def c(*args):
    if isinstance(args[0], list) or isinstance(args[0], tuple):
        first = args[0][0]
    else:
        first = args[0]

    if isinstance(first, int):
        out = rvec.RInt.new()
    elif isinstance(first, float):
        out = rvec.RFloat.new()
    elif isinstance(first, bool):
        out = rvec.RBool.new()
    elif isinstance(first, str):
        out = rvec.RString.new()
    else:
        raise TypeError("Unsupported type: {}".format(type(first)))

    for arg in args:
        out.push(arg)

    return out

__doc__ = rvec.__doc__
if hasattr(rvec, "__all__"):
    __all__ = rvec.__all__