from .rvec import *
# from rvec import new

print("running rvec/__init__.py")
print("rvec/__init__.py: dir(rvec) = ", dir(rvec))
print("rvec/__init__.py: dir(rvec.RVec) = ", dir(rvec.RVec))


__doc__ = rvec.__doc__
if hasattr(rvec, "__all__"):
    __all__ = rvec.__all__