import nlsd

def test_none():
    assert(nlsd.from_string("empty") == None)

def test_bool():
    assert(nlsd.from_string("true") == True)
    assert(nlsd.from_string("false") == False)

def test_string():
    assert(nlsd.from_string("`hello`") == "hello")

def test_num():
    assert(nlsd.from_string("1") == 1)
    assert(nlsd.from_string("1.2") == 1.2)
    assert(nlsd.from_string("-123") == -123)

def test_list():
    assert(nlsd.from_string("the list where an item is 1 and another item is 2 and another item is 3") == [1,2,3])

def test_object():
    assert(nlsd.from_string("the object where `a` is 1 and `b` is 2") == {"a": 1, "b": 2})

def test_nested():
    assert(nlsd.from_string("the list henceforth `the list` where an item is the object where `a` is 1 and another item of `the list` is the object where `b` is 2") == [{"a": 1}, {"b": 2}])
    assert(nlsd.from_string("the object henceforth `the object` where `a` is the list where an item is 1 and `b` of `the object` is the list where an item is 2") == {"a": [1], "b": [2]})
