import nlsd

def test_none():
    assert(nlsd.from_string(nlsd.to_string(None)) == None)

def test_bool():
    assert(nlsd.from_string(nlsd.to_string(True)) == True)
    assert(nlsd.from_string(nlsd.to_string(False)) == False)

def test_string():
    assert(nlsd.from_string(nlsd.to_string("hello")) == "hello")

def test_num():
    assert(nlsd.from_string(nlsd.to_string(1)) == 1)
    assert(nlsd.from_string(nlsd.to_string(1.2)) == 1.2)
    assert(nlsd.from_string(nlsd.to_string(-123)) == -123)

def test_list():
    assert(nlsd.from_string(nlsd.to_string([1,2,3])) == [1,2,3])

def test_object():
    assert(nlsd.from_string(nlsd.to_string({"a": 1, "b": 2})) == {"a": 1, "b": 2})

def test_nested():
    assert(nlsd.from_string(nlsd.to_string([{"a": 1}, {"b": 2}])) == [{"a": 1}, {"b": 2}])
    assert(nlsd.from_string(nlsd.to_string({"a": [1], "b": [2]})) == {"a": [1], "b": [2]})
