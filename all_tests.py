import unittest

if __name__ == '__main__':
    allTests = unittest.TestLoader().discover('src/test/tests_2019')

    unittest.TextTestRunner(verbosity=2).run(allTests)
