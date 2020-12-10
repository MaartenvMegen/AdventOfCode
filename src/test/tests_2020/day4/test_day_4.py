import os
import unittest

from src.utility import lineyielder

THIS_DIR = os.path.dirname(os.path.abspath(__file__))
import re

expected_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
optional_fields = ['cid']

eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


def validate_field_presence(passport):
    try:
        for field in expected_fields:
            _ = passport[field]
    except Exception as e:
        #print(f"missing required field: {e} for passport {passport}")
        return False
    return True


def validate_byr(passport):
    check = 1920 <= int(passport['byr']) <= 2002
    if not check:
        #print(f"invalid birthyear: {passport['byr']}")
        pass
    return check


def validate_iyr(passport):
    check = 2010 <= int(passport['iyr']) <= 2020
    if not check:
        # print(f"invalid issue year: {passport['iyr']}")
        pass
    return check


def validate_eyr(passport):
    check = 2020 <= int(passport['eyr']) <= 2030
    if not check:
        #print(f"invalid expiration year: {passport['eyr']}")
        pass
    return check


def validate_hgt(passport):
    pass_re = re.compile("\d{2,3}|cm|in")
    match = pass_re.findall(passport['hgt'])
    height = match[0]
    unit = match[1] if len(match) > 1 else ""
    check = False
    if unit == "cm":
        check = 150 <= int(height) <= 193
    elif unit == 'in':
        check = 59 <= int(height) <= 76
    if not check:
        #print(f"invalid height: {passport['hgt']}")
        pass
    return check


def validate_hcl(passport):
    pass_re = re.compile("#[a-f,0-9]{6}")
    match = pass_re.match(passport['hcl'])
    check = True if match else False
    if not check:
        #print(f"invalid hair color: {passport['hcl']}")
        pass
    return check


def validate_ecl(passport):
    check = passport['ecl'] in eye_colors
    if not check:
        # print(f"invalid eye color: {passport['ecl']}")
        pass
    return check


def validate_pid(passport):
    pass_re = re.compile("\d{9}")
    match = pass_re.fullmatch(passport['pid'])
    check = True if match else False

    if not check:
        # print(f"invalid pid: {passport['pid']}")
        pass
    return check


def get_valid_passports_from_file(filename, validators):
    passports = []
    passport = dict()
    evaluation_counter = 0

    for line in lineyielder.yield_lines(os.path.join(THIS_DIR, filename)):
        if not line:
            # store passport if valid and create a new one
            evaluation_counter += 1
            if validate_field_presence(passport) and all([validator(passport) for validator in validators]):
                passports.append(passport)
            passport = dict()
        else:
            keyValuePairs = re.split(' ', line)
            for keyValue in keyValuePairs:
                key, value = re.split(':', keyValue)
                passport[key] = value
    #print(f'evaluated {evaluation_counter} passports')
    return passports


class Day4Tester(unittest.TestCase):

    def test_year(self):
        passport = dict()
        passport['byr'] = "2002"
        self.assertEqual(True, validate_byr(passport))
        passport['byr'] = "1937"
        self.assertEqual(True, validate_byr(passport))
        passport['byr'] = "1900"
        self.assertEqual(False, validate_byr(passport))

    def test_height(self):
        passport = dict()
        passport['hgt'] = "190cm"
        self.assertEqual(True, validate_hgt(passport))

        passport['hgt'] = "60in"
        self.assertEqual(True, validate_hgt(passport))

        passport['hgt'] = "190in"
        self.assertEqual(False, validate_hgt(passport))

        passport['hgt'] = "190"
        self.assertEqual(False, validate_hgt(passport))

    def test_hcl(self):
        passport = dict()
        passport['hcl'] = "#ab34eb"
        self.assertEqual(True, validate_hcl(passport))
        passport['hcl'] = "#123abz"
        self.assertEqual(False, validate_hcl(passport))
        passport['hcl'] = "ab34eb"
        self.assertEqual(False, validate_hcl(passport))

    def test_ecl(self):
        passport = dict()
        passport['ecl'] = "amb"
        self.assertEqual(True, validate_ecl(passport))
        passport['ecl'] = "wat"
        self.assertEqual(False, validate_ecl(passport))

    def test_pid(self):
        passport = dict()
        passport['pid'] = "012345678"
        self.assertEqual(True, validate_pid(passport))
        passport['pid'] = "000000001"
        self.assertEqual(True, validate_pid(passport))
        passport['pid'] = "0123456789"
        self.assertEqual(False, validate_pid(passport))

    def test_part_a_example(self):
        passports = get_valid_passports_from_file('example.txt', [validate_field_presence])
        self.assertEqual(2, len(passports))

    def test_part_a(self):
        passports = get_valid_passports_from_file('input.txt', [validate_field_presence])
        self.assertEqual(254, len(passports))

    def test_part_b_valid_sample(self):
        validators = [validate_field_presence, validate_hcl, validate_byr, validate_ecl, validate_eyr, validate_hgt,
                      validate_iyr, validate_pid]
        passports = get_valid_passports_from_file('part_b_valid.txt', validators)
        self.assertEqual(4, len(passports))

    def test_part_b_invalid_sample(self):
        validators = [validate_field_presence, validate_hcl, validate_byr, validate_ecl, validate_eyr, validate_hgt,
                      validate_iyr, validate_pid]
        passports = get_valid_passports_from_file('part_b_invalid.txt', validators)
        self.assertEqual(0, len(passports))

    def test_part_b(self):
        validators = [validate_field_presence, validate_byr, validate_iyr, validate_eyr, validate_hgt, validate_hcl,
                      validate_ecl, validate_pid]
        passports = get_valid_passports_from_file('input.txt', validators)
        self.assertEqual(184, len(passports))
