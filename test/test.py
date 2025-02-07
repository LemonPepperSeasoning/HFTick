import sys
import signal
import time
import rscheduler


def my_task():
    print(f"Task running at {time.time()}")


def my_task2():
    print(f"Task running at {time.time()}")


def cleanup_and_exit(signum, frame):
    sys.exit(0)


if __name__ == "__main__":
    signal.signal(signal.SIGINT, cleanup_and_exit)

    scheduler = rscheduler.Scheduler()
    scheduler.schedule(my_task, 1.0)
    scheduler.schedule(my_task2, 1.0)

    scheduler.start()
    """
    scheduler = rscheduler.Scheduler()
    scheduler.schedule(my_task, 1.0)
    scheduler.schedule(my_task2, 1.0)
    scheduler.start()
    """

    while True:
        time.sleep(1)
