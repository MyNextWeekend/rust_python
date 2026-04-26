import threading
import time

from rust_python import parallel_sum_of_squares


def test_thread():
    number: int = 20000000
    count: int = 5

    # 测试多线程
    start_time = time.time()
    thread_list = []
    for _ in range(count):
        t = threading.Thread(target=parallel_sum_of_squares, args=(number,))
        t.start()
        thread_list.append(t)

    for thread in thread_list:
        thread.join()
    print(f"多线程耗时：{(time.time() - start_time):.6f}")

    start_time = time.time()
    for _ in range(count):
        print(parallel_sum_of_squares(num=number))
    print(f"单线程顺序耗时：{(time.time() - start_time):.6f}")


if __name__ == "__main__":
    test_thread()
