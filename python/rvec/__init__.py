from .rvec import *
import rvec

__doc__ = rvec.__doc__
if hasattr(rvec, "__all__"):
    __all__ = rvec.__all__

def c(*args):
    """Create a new RVec from arguments."""
    # TODO: support nested lists, tuples, and RVecs

    # other = []
    # rvecs = []
    # for arg in args:
    #     if isinstance(arg, rvec.RVec):
    #         rvecs.append(arg)
    #     elif isinstance(arg, list) or isinstance(arg, tuple):
    #         other.extend(arg)
    #     else:
    #         other.append(arg)
    
    # rvecs.append(rvec.RVec(other))

    # return rvec.concat(rvecs)
    return rvec.RVec(args)

print("running rvec/__init__.py")
print("rvec/__init__.py: dir(rvec) = ", dir(rvec))
print("rvec/__init__.py: dir(rvec.RVec) = ", dir(rvec.RVec))