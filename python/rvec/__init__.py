from .rvec import *
from rvec import new

print("running rvec/__init__.py")

def c(*args):
    out = []
    for arg in args:
        if isinstance(arg, list) or isinstance(arg, tuple):
            out += arg
        else:
            out.append(arg)

    return new(out)

__doc__ = rvec.__doc__
if hasattr(rvec, "__all__"):
    __all__ = rvec.__all__