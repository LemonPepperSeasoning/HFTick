[![PyPi](https://img.shields.io/pypi/v/rscheduler)](https://pypi.org/project/rscheduler/)
[![GitHub actions status](https://github.com/lemonpepperseasoning/rscheduler/workflows/CI/badge.svg)](https://github.com/lemonpepperseasoning/rscheduler/actions/workflows/CI.yml)

# rscheduler - A Python Scheduling Library in Rust

`rscheduler` is a Python library built with Rust using PyO3. It allows you to schedule Python functions to run at specific intervals and manage those tasks dynamically. This library is especially useful when you need high performance with Rust's concurrency, while still being able to interface with Python.

## Installation

To install the `rscheduler` library, you'll need to have Rust and Python installed on your system. You'll also need `maturin`, a tool to build Python packages from Rust.

### Prerequisites

- Python 3.6 or later
- Rust (with `cargo` and `rustc` installed)
- `maturin` tool (used for building the Python extension)

### Steps to Install

1. **Install maturin: You can install maturin via pip**:

   ```bash
   pip install maturin
   ```

2. **Build and install the package: After cloning this repository, build the package using maturin**:
   ```bash
   git clone https://github.com/LemonPepperSeasoning/rscheduler.git
   cd rscheduler
   maturin develop
   ```
   This will build and install the library into your Python environment.

### Usage

Once installed, you can start using the rscheduler library in your Python code. Below are some basic examples.

1. Import the rscheduler module

   ```
   import rscheduler
   ```

2. Create a scheduler instance

   ```
   scheduler = rscheduler.Scheduler()
   ```

3. Define a Python function to be scheduled

   ```
   def my_task():
       print("Task executed!")
   ```

4. Schedule the task with an interval (in seconds)
   You can schedule a task by passing the Python function and the interval:

   ```
   task_id = scheduler.schedule(my_task, 2.0) # Executes every 2 seconds
   ```

5. You can start the scheduler to begin executing the tasks:

   ```
   scheduler.start()
   ```

6. List all scheduled tasks
   You can list all the scheduled tasks, both running and pending:

   ```
   scheduler.list_schedules()
   ```

7. Terminate a task
   To stop a specific task, you can call the terminate method with the task's ID:

   ```
   scheduler.terminate(task_id)
   ```

8. Shut down the scheduler
   To stop all tasks and shut down the scheduler, use:

   ```
   scheduler.shutdown()
   ```

For any issues or feature requests, please open an issue on GitHub.

### License

Apache License 2.0

### TODO:

- Go lower level & integrate with syscall (look at psutil for api example)
- Sphinx integration for Documentation
- fix dependency. (should be `pyo3 = { version = "0.23.4", features = ["extension-module"] }` i think)

#### Release

```
Update Cargo.toml version
Update CHANGELOG.md
git commit -m "Release vX.Y.Z"
git tag -a vX.Y.Z -m "Version X.Y.Z"
git push origin vX.Y.Z
```
