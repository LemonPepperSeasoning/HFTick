use pyo3::prelude::*;
use pyo3::types::PyFunction;
use std::thread;
use std::time::Duration;

/// Runs a given Python function every `interval` seconds in a separate thread.
#[pyfunction]
fn run_scheduler(py_func: PyObject, interval: f64) -> PyResult<()> {
    thread::spawn(move || {
        loop {
            // Acquire GIL before calling the Python function
            Python::with_gil(|py| {
                if let Err(err) = py_func.call0(py) {
                    eprintln!("Error calling Python function: {:?}", err);
                }
            });

            thread::sleep(Duration::from_secs_f64(interval));
        }
    });

    Ok(())
}

/// Create Python bindings
#[pymodule]
fn rscheduler(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_scheduler, m)?)?;
    Ok(())
}
