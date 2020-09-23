#[macro_use]
extern crate cpython;

use cpython::PyBytes;
use cpython::{PyObject, PyResult, Python};
use std::io::Write;

py_exception!(nlsd, SerializeError);

py_exception!(nlsd, DeserializeError);

fn from_string(py: Python, string: String) -> PyResult<PyObject> {
    let mut deserializer = nlsd::Deserializer::from_str(&string);

    let mut out = Vec::new();
    out.write_all(&[b'\x80', b'\x03'])
        .map_err(|e| DeserializeError::new(py, e.to_string()))?; // PROTO v3
    let mut serializer = serde_pickle::Serializer::new(&mut out, false);
    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .map_err(|e| DeserializeError::new(py, e.to_string()))?;
    out.write_all(&[b'.'])
        .map_err(|e| DeserializeError::new(py, e.to_string()))?; // STOP

    let obj = py
        .import("pickle")?
        .call(py, "loads", (PyBytes::new(py, &out),), None)?;
    obj.extract(py)
}

fn to_string(py: Python, obj: PyObject) -> PyResult<String> {
    let pickled = py.import("pickle")?.call(py, "dumps", (obj,), None)?;
    let mut deserializer =
        serde_pickle::Deserializer::new(pickled.cast_as::<PyBytes>(py)?.data(py), true);
    let mut out = String::new();
    let mut serializer = nlsd::Serializer::new(&mut out);
    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .map_err(|e| SerializeError::new(py, e.to_string()))?;
    Ok(out)
}

py_module_initializer!(nlsd, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "from_string", py_fn!(py, from_string(string: String)))?;
    m.add(py, "to_string", py_fn!(py, to_string(obj: PyObject)))?;
    Ok(())
});
