import unittest
import rvec

class TestRVec(unittest.TestCase):
    def test_new(self):
        # Test creating an RVec from a list of values
        r = rvec.RVec([1, 2, 3])
        self.assertEqual(list(r), [1, 2, 3])
        
    def test_str(self):
        # Test converting an RVec to a string
        r = rvec.RVec([1, 2, 3])
        self.assertEqual(str(r), "[1, 2, 3]")
        
    def test_len(self):
        # Test getting the length of an RVec
        r = rvec.RVec([1, 2, 3])
        self.assertEqual(len(r), 3)
        
    def test_add(self):
        # Test adding two RVecs
        r1 = rvec.RVec([1, 2, 3])
        r2 = rvec.RVec([4, 5, 6])
        r3 = r1 + r2
        self.assertEqual(list(r3), [5, 7, 9])
        
        # Test adding an RVec and a scalar
        r4 = r1 + 1
        self.assertEqual(list(r4), [2, 3, 4])
        
    def test_sub(self):
        # Test subtracting two RVecs
        r1 = rvec.RVec([1, 2, 3])
        r2 = rvec.RVec([4, 5, 6])
        r3 = r2 - r1
        self.assertEqual(list(r3), [3, 3, 3])
        
        # Test subtracting an RVec and a scalar
        r4 = r2 - 1
        self.assertEqual(list(r4), [3, 4, 5])
        
    def test_mul(self):
        # Test multiplying two RVecs
        r1 = rvec.RVec([1, 2, 3])
        r2 = rvec.RVec([4, 5, 6])
        r3 = r1 * r2
        self.assertEqual(list(r3), [4, 10, 18])
        
        # Test multiplying an RVec and a scalar
        r4 = r1 * 2
        self.assertEqual(list(r4), [2, 4, 6])
        
    def test_truediv(self):
        # Test dividing two RVecs
        r1 = rvec.RVec([1, 2, 3])
        r2 = rvec.RVec([4, 5, 6])
        r3 = r2 / r1
        self.assertEqual(list(r3), [4, 2.5, 2])
        
        # Test dividing an RVec and a scalar
        r4 = r1 / 2
        self.assertEqual(list(r4), [0.5, 1, 1.5])

if __name__ == '__main__':
    unittest.main()