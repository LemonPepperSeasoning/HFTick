use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

use crate::Subroutine;

pub struct Task {
    pub id: i8,
    pub subroutine: Subroutine,
    pub interval: f64,
}

#[pyclass(module = "rscheduler")]
pub struct Scheduler {
    subroutines: Vec<Task>,
    switches: HashMap<i8, Arc<AtomicBool>>,
    counter: i8,
}

#[pymethods]
impl Scheduler {
    #[new]
    fn new() -> Self {
        Self {
            subroutines: Vec::new(),
            switches: HashMap::new(),
            counter: 0,
        }
    }

    pub fn schedule(&mut self, py_func: PyObject, interval: f64) -> PyResult<i8> {
        self.counter += 1;
        self.subroutines.push(Task {
            id: self.counter,
            subroutine: Subroutine { py_func },
            interval,
        });
        self.switches
            .insert(self.counter, Arc::new(AtomicBool::new(true)));
        Ok(self.counter)
    }

    pub fn start(&mut self) -> PyResult<()> {
        for task in self.subroutines.drain(..) {
            let task_id = task.id;
            let switch = self.switches.get(&task_id).unwrap().clone();
            let _ = thread::spawn(move || {
                let start_time = Instant::now();
                let mut counter = 0;

                while switch.load(Ordering::Relaxed) {
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
                println!("Task id:{} stopped.", task_id);
            });
        }
        Ok(())
    }

    pub fn terminate(&mut self, task_id: i8) -> PyResult<()> {
        // cancel a subroutine
        if let Some(switch) = self.switches.get(&task_id) {
            switch.store(false, Ordering::Relaxed);
            self.switches.remove(&task_id);
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Task id:{} not found",
                task_id
            )));
        }
        Ok(())
    }

    pub fn list_schedules(&self) -> PyResult<()> {
        // list all scheduled subroutines
        println!("On backlog");
        for task in &self.subroutines {
            println!("Task id:{} yet to be scheduled", task.id);
        }

        println!("Running");
        for (task_id, switch) in &self.switches {
            if switch.load(Ordering::Relaxed) {
                println!("Task id:{} is running", task_id);
            }
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> PyResult<()> {
        for switch in self.switches.values() {
            switch.store(false, Ordering::Relaxed);
        }
        self.switches.clear();
        self.subroutines.clear();
        Ok(())
    }
}
