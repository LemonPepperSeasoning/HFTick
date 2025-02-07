use pyo3::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;

use crate::Subroutine;

pub struct Task {
    pub subroutine: Subroutine,
    pub interval: f64,
}


#[pyclass(module = "rscheduler")]
pub struct Scheduler {
    subroutines: Vec<Task>,
}

#[pymethods]
impl Scheduler {

    #[new]
    fn new() -> Self {
        Self { subroutines: Vec::new(), }
    }

    pub fn schedule(&mut self, py_func: PyObject, interval: f64) -> PyResult<()> {
        self.subroutines.push(
            Task {
                subroutine: Subroutine { py_func },
                interval
            }
        );
        Ok(())
    }

    pub fn start(&mut self) -> PyResult<()> {
        let task_handles: Vec<_> = self.subroutines.drain(..)
            .map(|task| {
                thread::spawn(move || {
                    let start_time = Instant::now();
                    let mut counter = 0;

                    loop {
                        Python::with_gil(|py| {
                            if let Err(err) = task.subroutine.run(py) {
                                eprintln!("Error calling Python function: {:?}", err);
                            }
                        });

                        counter += 1;
                        let elapsed_time = Instant::now().duration_since(start_time).as_secs_f64();
                        let sleep_time = (task.interval * counter as f64) - elapsed_time;

                        if sleep_time > 0.0 {
                            thread::sleep(Duration::from_secs_f64(sleep_time));
                        }
                    }
                })
            })
            .collect();

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