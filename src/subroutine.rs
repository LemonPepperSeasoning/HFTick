use pyo3::prelude::*;


pub struct Subroutine {
    pub py_func: PyObject,
}

impl Subroutine {

    pub fn run(&self, py: Python) -> PyResult<PyObject> {
        self.py_func.call0(py)
    }

    fn get_next_run_time(&self) -> f64 {
        0.0
    }
}

impl Clone for Subroutine {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Subroutine {
            py_func: self.py_func.clone_ref(py),
        })
    }
}



pub struct Configuration {
    pub interval: f64,
    pub repeat: bool,
    pub start_time: f64,
    pub start_immediately: bool,
    pub end_time: f64,
    pub weekdays: Vec<u8>,
    pub monthdays: Vec<u8>,
    pub months: Vec<u8>,
    pub years: Vec<u16>,

    pub priority: u8,
}