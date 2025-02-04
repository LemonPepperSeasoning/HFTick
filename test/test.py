import time
import rscheduler


def my_task():
    print(f"Task running at {time.time()}")


rscheduler.run_scheduler(my_task, 2.0)  # Run every 2 seconds

while True:
    time.sleep(1)
