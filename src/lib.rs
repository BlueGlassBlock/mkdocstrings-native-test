use pyo3::prelude::*;
use pyo3_asyncio::{tokio::future_into_py_with_locals, TaskLocals};

#[pyfunction]
fn async_add(a: i64, b: i64) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let locals = TaskLocals::with_running_loop(py)?;
        Ok(future_into_py_with_locals(py, locals, async move { Ok(a + b) })?.into_py(py))
    })
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass(module = "async_trash.core")]
struct Adder {
    #[pyo3(get)]
    param: i64,
}

#[pymethods]
impl Adder {
    #[new]
    // Init self
    pub fn new(param: i64) -> Self {
        Self { param }
    }

    pub fn add(&self, other: i64) -> i64 {
        self.param + other
    }

    pub fn async_add(&self, other: i64) -> PyResult<PyObject> {
        let param = self.param;
        Python::with_gil(|py| {
            let locals = TaskLocals::with_running_loop(py)?;
            Ok(
                future_into_py_with_locals(py, locals, async move { Ok(param + other) })?
                    .into_py(py),
            )
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(async_add, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Adder>()?;
    Ok(())
}
