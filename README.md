# nlsd-py

A [Natural Language Structured Document](https://github.com/RiddleAndCode/nlsd/tree/master/nlsd) serializer and deserializer python wrappers. This library uses Rust bindings under the hood.

## Usage

```python
import nlsd

st = nlsd.to_string([1, "hello"])
print(st) # the list where an item is 1 and another item is `hello`

obj = nlsd.from_string(st)
print(obj) # [1, "hello"]
```
