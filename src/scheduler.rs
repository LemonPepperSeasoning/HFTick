use pyo3::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use crate::Subroutine;

#[pyclass(module = "rscheduler")]
pub struct Scheduler {
}

#[pymethods]
impl Scheduler {

    #[new]
    fn new() -> Self {
        Self {  }
    }

    pub fn schedule(&self, py_func: PyObject, interval: f64) -> PyResult<()> {
        // schedule a new subroutine
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

    pub fn cancel(&self) -> PyResult<()> {
        // cancel a subroutine
        Ok(())
    }

    pub fn list_schedules(&self) -> PyResult<()> {
        // list all scheduled subroutines
        Ok(())
    }

    pub fn shutdown(&self) -> PyResult<()> {
        // shutdown the scheduler
        Ok(())
    }
}