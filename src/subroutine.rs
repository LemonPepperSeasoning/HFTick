use pyo3::prelude::*;

#[pyclass(module = "rscheduler")]
pub struct Subroutine {
    py_func: PyObject,
}

#[pymethods]
impl Subroutine {
    #[new]
    fn new(py_func: PyObject) -> Self {
        Self { py_func }
    }

    fn run(&self, py: Python) -> PyResult<PyObject> {
        self.py_func.call0(py)
    }

    fn get_next_run_time(&self) -> f64 {
        0.0
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