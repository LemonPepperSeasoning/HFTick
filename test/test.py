import time
import rscheduler


def my_task():
    print(f"Task running at {time.time()}")


rscheduler.run_scheduler(my_task, 1.0)

while True:
    time.sleep(1)
