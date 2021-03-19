use crate::error::*;
use cpython::{PyDict, PyObject, PyResult, PyString, Python};

pub fn from_string(py: Python, string: String, de_opts: Option<&PyDict>) -> PyResult<PyObject> {
    let mut deserializer = nlsd::Deserializer::from_str(&string);

    let mut out = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut out);
    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .map_err(|e| DeserializeError::new(py, e.to_string()))?;
    let out_str = String::from_utf8(out).map_err(|e| DeserializeError::new(py, e.to_string()))?;

    let obj = py
        .import("json")?
        .call(py, "loads", (PyString::new(py, &out_str),), de_opts)?;
    obj.extract(py)
}

pub fn to_string(py: Python, obj: PyObject, ser_opts: Option<&PyDict>) -> PyResult<String> {
    let json = py.import("json")?.call(py, "dumps", (obj,), ser_opts)?;
    let json_str = json.cast_as::<PyString>(py)?.data(py).to_string(py)?;
    let mut deserializer = serde_json::Deserializer::from_reader(json_str.as_bytes());
    let mut out = String::new();
    let mut serializer = nlsd::Serializer::new(&mut out);
    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .map_err(|e| SerializeError::new(py, e.to_string()))?;
    Ok(out)
}
