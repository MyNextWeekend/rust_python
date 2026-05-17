import threading
import time

from rust_python import (
    parallel_average,
    parallel_filter_even,
    parallel_max,
    parallel_sum_array,
    parallel_sum_of_squares,
    parallel_square_array,
    parallel_with_thread_pool,
    generate_large_array,
)


def test_basic_parallel_sum():
    """基本并行求和测试"""
    number: int = 20_000_000
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
    print(f"多线程并行求和耗时：{(time.time() - start_time):.6f}s")

    # 测试单线程顺序执行
    start_time = time.time()
    for _ in range(count):
        parallel_sum_of_squares(num=number)
    print(f"单线程顺序求和耗时：{(time.time() - start_time):.6f}s")


def test_parallel_array_operations():
    """并行数组操作测试"""
    # 生成测试数据
    arr = list(range(1, 1_000_001))
    arr_f64 = list(map(float, arr))
    
    # 测试并行平方数组
    start = time.time()
    squared = parallel_square_array(arr)
    print(f"parallel_square_array 耗时: {(time.time() - start):.6f}s")
    print(f"  结果长度: {len(squared)}, 前5个元素: {squared[:5]}")
    
    # 测试并行求和
    start = time.time()
    total = parallel_sum_array(arr)
    print(f"parallel_sum_array 耗时: {(time.time() - start):.6f}s")
    print(f"  结果: {total}")
    
    # 测试并行过滤偶数
    start = time.time()
    evens = parallel_filter_even(arr)
    print(f"parallel_filter_even 耗时: {(time.time() - start):.6f}s")
    print(f"  结果长度: {len(evens)}, 前5个元素: {evens[:5]}")
    
    # 测试并行最大值
    start = time.time()
    max_val = parallel_max(arr)
    print(f"parallel_max 耗时: {(time.time() - start):.6f}s")
    print(f"  结果: {max_val}")
    
    # 测试并行平均值
    start = time.time()
    avg = parallel_average(arr_f64)
    print(f"parallel_average 耗时: {(time.time() - start):.6f}s")
    print(f"  结果: {avg:.2f}")


def test_thread_pool_control():
    """线程池控制测试"""
    number = 10_000_000
    
    for threads in [1, 2, 4, 8]:
        start = time.time()
        result = parallel_with_thread_pool(number, threads)
        elapsed = time.time() - start
        print(f"parallel_with_thread_pool (threads={threads}) 耗时: {elapsed:.6f}s")
        print(f"  结果: {result}")


def test_large_data_processing():
    """大数据处理测试"""
    print("\n生成大数据...")
    large_arr = generate_large_array(10_000_000)
    print(f"数据长度: {len(large_arr)}")
    
    # 在大数据上测试并行操作
    start = time.time()
    total = parallel_sum_array(large_arr)
    print(f"parallel_sum_array (10M elements): {(time.time() - start):.6f}s")
    print(f"  总和: {total}")
    
    start = time.time()
    max_val = parallel_max(large_arr)
    print(f"parallel_max (10M elements): {(time.time() - start):.6f}s")
    print(f"  最大值: {max_val}")


def test_concurrent_threads():
    """并发线程测试"""
    number = 5_000_000
    thread_count = 4
    
    def worker(thread_id):
        print(f"线程 {thread_id} 开始计算")
        start = time.time()
        result = parallel_sum_of_squares(number)
        elapsed = time.time() - start
        print(f"线程 {thread_id} 完成，耗时: {elapsed:.4f}s，结果: {result}")
        return result
    
    threads = []
    start_time = time.time()
    
    for i in range(thread_count):
        t = threading.Thread(target=worker, args=(i,))
        threads.append(t)
        t.start()
    
    for t in threads:
        t.join()
    
    total_time = time.time() - start_time
    print(f"\n所有线程完成，总耗时: {total_time:.4f}s")


if __name__ == "__main__":
    print("=" * 70)
    print("测试基本并行求和")
    print("=" * 70)
    test_basic_parallel_sum()
    
    print("\n" + "=" * 70)
    print("测试并行数组操作")
    print("=" * 70)
    test_parallel_array_operations()
    
    print("\n" + "=" * 70)
    print("测试线程池控制")
    print("=" * 70)
    test_thread_pool_control()
    
    print("\n" + "=" * 70)
    print("测试大数据处理")
    print("=" * 70)
    test_large_data_processing()
    
    print("\n" + "=" * 70)
    print("测试并发线程")
    print("=" * 70)
    test_concurrent_threads()