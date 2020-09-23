import nlsd

def test_string():
    assert(nlsd.to_string("hello") == "`hello`")

def test_num():
    assert(nlsd.to_string(1) == "1")
    assert(nlsd.to_string(1.2) == "1.2")
    assert(nlsd.to_string(-123) == "-123")

def test_list():
    assert(nlsd.to_string([1,2,3]) == "the list where an item is 1 and another item is 2 and another item is 3")
