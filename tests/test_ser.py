import nlsd

def test_none():
    assert(nlsd.to_string(None) == "empty")

def test_bool():
    assert(nlsd.to_string(True) == "true")
    assert(nlsd.to_string(False) == "false")

def test_string():
    assert(nlsd.to_string("hello") == "`hello`")

def test_num():
    assert(nlsd.to_string(1) == "1")
    assert(nlsd.to_string(1.2) == "1.2")
    assert(nlsd.to_string(-123) == "-123")

def test_list():
    assert(nlsd.to_string([1,2,3]) == "the list where an item is 1 and another item is 2 and another item is 3")

def test_object():
    assert(nlsd.to_string({"a": 1, "b": 2}) == "the object where `a` is 1 and `b` is 2")

def test_nested():
    assert(nlsd.to_string([{"a": 1}, {"b": 2}]) == "the list henceforth `the list` where an item is the object where `a` is 1 and another item of `the list` is the object where `b` is 2")
    assert(nlsd.to_string({"a": [1], "b": [2]}) == "the object henceforth `the object` where `a` is the list where an item is 1 and `b` of `the object` is the list where an item is 2")
