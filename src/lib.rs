use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{create_exception, wrap_pyfunction};
use std::io::Write;

create_exception!(nlsd, SerializeError, PyException);

create_exception!(nlsd, DeserializeError, PyException);

/// Serializes a python object into an NLSD string
#[pyfunction]
fn to_string(py: Python, obj: PyObject) -> PyResult<String> {
    let pickled: &PyBytes = py.import("pickle")?.call1("dumps", (obj,))?.downcast()?;
    let mut deserializer = serde_pickle::Deserializer::new(pickled.as_bytes(), true);
    let mut out = String::new();
    let mut serializer = nlsd::Serializer::new(&mut out);
    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .map_err(|e| SerializeError::new_err(e.to_string()))?;
    Ok(out)
}

/// Deserializes an NLSD string into a pyton object
#[pyfunction]
fn from_string(py: Python, string: String) -> PyResult<PyObject> {
    let mut deserializer = nlsd::Deserializer::from_str(&string);

    let mut out = Vec::new();
    out.write_all(&[b'\x80', b'\x03'])?; // PROTO v3
    let mut serializer = serde_pickle::Serializer::new(&mut out, false);
    serde_transcode::transcode(&mut deserializer, &mut serializer)
        .map_err(|e| DeserializeError::new_err(e.to_string()))?;
    out.write_all(&[b'.'])?; // STOP

    let bytes = unsafe { PyBytes::from_ptr(py, out.as_ptr(), out.len()) };
    let obj = py.import("pickle")?.call1("loads", (bytes,))?.extract()?;
    Ok(obj)
}

/// A Python module with the functions to deserialize from and serialize to a string
#[pymodule]
fn nlsd(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("SerializeError", py.get_type::<SerializeError>())?;
    m.add("DeserializeError", py.get_type::<DeserializeError>())?;
    m.add_function(wrap_pyfunction!(to_string, m)?)?;
    m.add_function(wrap_pyfunction!(from_string, m)?)?;
    Ok(())
}
