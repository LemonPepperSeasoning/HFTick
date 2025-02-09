[![PyPi](https://img.shields.io/pypi/v/rscheduler)](https://pypi.org/project/rscheduler/)
[![GitHub actions status](https://github.com/lemonpepperseasoning/rscheduler/workflows/CI/badge.svg)](https://github.com/lemonpepperseasoning/rscheduler/actions/workflows/CI.yml)

# rscheduler

python scheduling library implemented in rust

### Project setup

```
python3 -m venv venv
source venv/bin/activate
pip install maturin
maturin develop
```

### Run

```
maturin develop
python3

>> import rscheduler
>> scheduler = rscheduler.Scheduler()
>> id1 = scheduler.schedule(my_task, 1.0)
>> scheduler.start()
>> time.sleep(3)
>> scheduler.cancel(id1)

```

### Test

```
cargo test
```

### TODO:

- Go lower level & integrate with syscall (look at psutil for api example)

### Contribution guideline

...

#### Release

```
Update Cargo.toml version
Update CHANGELOG.md
git commit -m "Release vX.Y.Z"
git tag -a vX.Y.Z -m "Version X.Y.Z"
git push origin vX.Y.Z
```
