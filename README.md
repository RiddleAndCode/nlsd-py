# nlsd-py

A [Natural Language Structured Document](https://github.com/RiddleAndCode/nlsd/tree/master/nlsd) serializer and deserializer python wrappers. This library uses Rust bindings under the hood.

## Usage

```python
import nlsd

str = nlsd.to_string([1, "hello"])
print(str) // the list where an item is 1 and another item is `hello`

obj = nlsd.from_string(str)
print(obj) // [1, "hello"]
```
