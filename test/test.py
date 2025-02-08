import sys
import signal
import time
import rscheduler


def my_task():
    print(f"Task 1: {time.time()}")


def my_task2():
    print(f"Task 2: {time.time()}")


def my_task3():
    print(f"Task 3: {time.time()}")


def cleanup_and_exit(signum, frame):
    sys.exit(0)


if __name__ == "__main__":
    signal.signal(signal.SIGINT, cleanup_and_exit)

    scheduler = rscheduler.Scheduler()
    id1 = scheduler.schedule(my_task, 1.0)
    print(id1)
    id2 = scheduler.schedule(my_task2, 1.0)
    print(id2)
    scheduler.start()

    time.sleep(3)

    scheduler.schedule(my_task3, 1.0)
    scheduler.start()

    time.sleep(3)
    scheduler.terminate(id1)

    time.sleep(3)

    scheduler.shutdown()
    print("Done")
