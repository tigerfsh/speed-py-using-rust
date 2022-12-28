import re 
import string 
import random 

from pyext_myrustlib import count_doubles as count_doubles_rust
from pyext_myrustlib import count_doubles_once as count_doubles_rust_once
from pyext_myrustlib import count_doubles_once_bytes

def count_doubles(val):
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total 

def count_doubles_once(val):
    total = 0
    chars = iter(val)
    c1 = next(chars)
    for c2 in chars:
        if c1 == c2:
            total += 1
        c1 = c2 
    return total

double_re = re.compile(r"(?=(.)\1)")

def count_doubles_regex(val):
    return len(double_re.findall(val))

val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))

def test_pure_python(benchmark):
    benchmark(count_doubles, val)

def test_regex(benchmark):
    benchmark(count_doubles_regex, val)

def test_rust(benchmark):
    benchmark(count_doubles_rust, val)

def test_rust_once(benchmark):
    benchmark(count_doubles_rust_once, val)

def test_rust_once_bytes(benchmark):
    benchmark(count_doubles_once_bytes, val)

def test_pure_python_once(benchmark):
    benchmark(count_doubles_once, val)


if __name__ == "__main__":
    print(count_doubles(val))
    print(count_doubles_regex(val))
    print(count_doubles_rust(val))
    print(count_doubles_once(val))
    print(count_doubles_rust_once(val))
    print(count_doubles_once_bytes(val))
