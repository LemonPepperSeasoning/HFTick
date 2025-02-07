use pyo3::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

mod scheduler;
pub use scheduler::*;
mod subroutine;
pub use subroutine::*;


#[pymodule]
fn rscheduler(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scheduler>()?;
    m.add_class::<Subroutine>()?;

    Ok(())
}
