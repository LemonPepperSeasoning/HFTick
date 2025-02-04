use pyo3::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

/// Runs a Python function at a fixed interval, adjusting for execution time.
#[pyfunction]
fn run_scheduler(py_func: PyObject, interval: f64) -> PyResult<()> {
    thread::spawn(move || {
        let start_time = Instant::now();
        let mut counter = 0;

        loop {
            Python::with_gil(|py| {
                if let Err(err) = py_func.call0(py) {
                    eprintln!("Error calling Python function: {:?}", err);
                }
            });

            counter += 1;
            let elapsed_time = Instant::now().duration_since(start_time).as_secs_f64();
            let sleep_time = (interval * counter as f64) - elapsed_time;

            // println!("elapsed_time: {}, sleep_time: {}", elapsed_time, sleep_time);
            if sleep_time > 0.0 {
                thread::sleep(Duration::from_secs_f64(sleep_time));
            }
        }
    });

    Ok(())
}

/// Define Python module
#[pymodule]
fn rscheduler(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_scheduler, m)?)?;
    Ok(())
}
