import unittest

if __name__ == '__main__':
    allTests = unittest.TestLoader().discover('src/test/tests_2019')

    result = unittest.TextTestRunner(verbosity=2).run(allTests)

    if result.wasSuccessful():
        exit(0)
    else:
        exit(1)
