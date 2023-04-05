from .rvec import *
# from rvec import new

print("running rvec/__init__.py")

__doc__ = rvec.__doc__
if hasattr(rvec, "__all__"):
    __all__ = rvec.__all__