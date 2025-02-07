use pyo3::prelude::*;

mod scheduler;
pub use scheduler::*;
mod subroutine;
pub use subroutine::*;


#[pymodule]
fn rscheduler(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scheduler>()?;
    Ok(())
}
