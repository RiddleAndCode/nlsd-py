#[cfg(all(feature = "json", feature = "pickle"))]
compile_error!("the `json` and `pickle` feature cannot both be enabled");

#[cfg(not(any(feature = "json", feature = "pickle")))]
compile_error!("either the `json` or `pickle` feature must be enabled");

#[macro_use]
extern crate cpython;

mod error;

#[cfg(feature = "pickle")]
#[path = "./pickle.rs"]
mod backend;

#[cfg(feature = "json")]
#[path = "./json.rs"]
mod backend;

use backend::*;

use cpython::{PyDict, PyObject};

py_module_initializer!(nlsd, |py, m| {
    m.add(py, "__doc__", "NLSD serializer / deserializer")?;
    m.add(
        py,
        "from_string",
        py_fn!(
            py,
            from_string(string: String, de_opts: Option<&PyDict> = None)
        ),
    )?;
    m.add(
        py,
        "to_string",
        py_fn!(
            py,
            to_string(obj: PyObject, ser_opts: Option<&PyDict> = None)
        ),
    )?;
    Ok(())
});
