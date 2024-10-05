import threading
import time

from rust_python import parallel_sum_of_squares

# 测试多线程耗时
start = time.time()
thread_list = []
for i in range(3):
    t = threading.Thread(target=parallel_sum_of_squares, args=(20000000,))
    t.start()
    thread_list.append(t)

for thread in thread_list:
    thread.join()
print(f"多线程耗时：{(time.time() - start):.6f}")

start = time.time()
print(parallel_sum_of_squares(20000000))
print(parallel_sum_of_squares(20000000))
print(parallel_sum_of_squares(20000000))
print(f"单线程顺序耗时：{(time.time() - start):.6f}")

