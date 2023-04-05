import unittest
from rvec import FloatVec
from rvec import IntVec
from rvec import BoolVec
from rvec import StrVec

class FloatTestCase(unittest.TestCase):
    def assertEqualVec(self, vec1, vec2):
        self.assertEqual(vec1.to_list(), vec2.to_list())

    def setUp(self):
        self.vec1 = FloatVec([1., 2., 3.])
        self.vec2 = FloatVec([4., 5., 6.])
        self.vec3 = FloatVec([0., 0., 0.])

    def test_add(self):
        result = self.vec1.add(self.vec2)
        self.assertEqual(result.to_list(), [5., 7., 9.])
        result = self.vec1 + self.vec2
        self.assertEqual(result.to_list(), [5., 7., 9.])

    def test_scalar_add(self):
        result = self.vec1.scalar_add(2)
        self.assertEqual(result.to_list(), [3., 4., 5.])
        result = self.vec1 + 2
        self.assertEqual(result.to_list(), [3., 4., 5.])

    def test_subtract(self):
        result = self.vec1.subtract(self.vec2)
        self.assertEqual(result.to_list(), [-3., -3., -3.])
        result = self.vec1 - self.vec2
        self.assertEqual(result.to_list(), [-3., -3., -3.])

    def test_scalar_subtract(self):
        result = self.vec1.scalar_subtract(2)
        self.assertEqual(result.to_list(), [-1., 0., 1.])
        result = self.vec1 - 2
        self.assertEqual(result.to_list(), [-1., 0., 1.])

    def test_multiply(self):
        result = self.vec1.multiply(self.vec2)
        self.assertEqual(result.to_list(), [4., 10., 18.])
        result = self.vec1 * self.vec2
        self.assertEqual(result.to_list(), [4., 10., 18.])

    def test_scalar_multiply(self):
        result = self.vec1.scalar_multiply(2)
        self.assertEqual(result.to_list(), [2., 4., 6.])
        result = self.vec1 * 2
        self.assertEqual(result.to_list(), [2., 4., 6.])

    def test_divide(self):
        result = self.vec1.divide(self.vec2)
        self.assertEqual(result.to_list(), [0.25, 0.4, 0.5])
        result = self.vec1 / self.vec2
        self.assertEqual(result.to_list(), [0.25, 0.4, 0.5])

    def test_scalar_divide(self):
        result = self.vec1.scalar_divide(2)
        self.assertEqual(result.to_list(), [0.5, 1.0, 1.5])
        result = self.vec1 / 2
        self.assertEqual(result.to_list(), [0.5, 1.0, 1.5])

    def test_indexing(self):
        self.assertEqual(self.vec1[0], 1.)
        self.assertEqual(self.vec1[1], 2.)
        self.assertEqual(self.vec1[2], 3.)
        self.assertEqual(self.vec1[-1], 3.)
        self.assertEqual(self.vec1[-2], 2.)
        self.assertEqual(self.vec1[-3], 1.)
    
    def test_slicing(self):
        self.assertEqual(self.vec1[0:2].to_list(), [1., 2.])
        self.assertEqual(self.vec1[1:3].to_list(), [2., 3.])
        self.assertEqual(self.vec1[0:3].to_list(), [1., 2., 3.])
        self.assertEqual(self.vec1[0:-1].to_list(), [1., 2.])
        self.assertEqual(self.vec1[1:-1].to_list(), [2.])
        self.assertEqual(self.vec1[0:-2].to_list(), [1.])


    def test_indexing_assignment(self):
        self.vec1[0] = 4.
        self.vec1[1] = 5.
        self.vec1[2] = 6.
        self.assertEqual(self.vec1.to_list(), [4., 5., 6.])

    # def test_iteration(self):
    #     for i, val in enumerate(self.vec1):
    #         self.assertEqual(val, i+1)

    def test_len(self):
        self.assertEqual(len(self.vec1), 3)

    def test_zeros(self):
        result = FloatVec.zeros(3)
        self.assertEqual(result.to_list(), [0., 0., 0.])



if __name__ == '__main__':
    unittest.main()
