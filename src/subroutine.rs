use pyo3::prelude::*;

pub struct Subroutine {
    pub py_func: PyObject,
}

impl Subroutine {
    pub fn run(&self, py: Python) -> PyResult<PyObject> {
        self.py_func.call0(py)
    }
}

impl Clone for Subroutine {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Subroutine {
            py_func: self.py_func.clone_ref(py),
        })
    }
}

pub struct ScheduleExpression {
    pub interval: Interval,
    pub n_repeat: i32, // -1 means infinite

    pub start_time: f64,
    pub start_immediately: bool,
    pub end_time: f64,
}

pub struct Interval {
    pub seconds: u16,
    pub minutes: u16,
    pub hours: u16,
    pub days: u16,
}

impl Interval {
    pub fn new(seconds: u16, minutes: u16, hours: u16, days: u16) -> Self {
        Self {
            seconds,
            minutes,
            hours,
            days,
        }
    }

    pub fn to_seconds(&self) -> u64 {
        self.seconds as u64
            + (self.minutes as u64) * 60
            + (self.hours as u64) * 3600
            + (self.days as u64) * 86400
    }
}
