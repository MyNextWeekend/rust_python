import time

from tests.log_utils import Logger
from rust_python import (
    ChildErrorA,
    Student,
    Teacher,
    ValidationError,
    chunk_array,
    dic_to_list,
    factorial,
    fibonacci,
    flatten_array,
    generate_large_array,
    hash_string,
    list_to_dic,
    many_args,
    reverse_string,
    student_info,
    unique_sorted,
)

log = Logger().get_logger()


def test_class():
    """类、方法调用示例"""
    stu = Student("里斯", 9, "三年级", 3.8)
    log.info(f"name:{stu.name},age:{stu.age},grade:{stu.grade},gpa:{stu.gpa}")
    age_list = [1, 2, 3, 4, 50, 10]
    max_age = stu.set_large_age(age_list)
    log.info(f"设置最大年龄: {max_age}, 当前年龄: {stu.age}")

    try:
        stu.raise_exception(-2)
    except ChildErrorA as e:
        log.info(f"catch ChildErrorA: {e}")
    except Exception as e:
        log.error(f"catch Exception: {e}")

    # 测试 calculate_birth_year
    birth_year = stu.calculate_birth_year()
    log.info(f"出生年份: {birth_year}")

    # 测试 get_full_description
    desc = stu.get_full_description()
    log.info(f"完整描述: {desc}")

    # 测试 update_info
    stu.update_info(name="李四")
    log.info(f"更新后姓名: {stu.name}")


def test_class_from_dict():
    """从字典创建学生实例"""
    data = {"name": "王五", "age": "20", "grade": "大学", "gpa": "3.9"}
    stu = Student.from_dict(data)
    log.info(f"从字典创建: {stu}")


def test_class_from_default():
    """从默认创建学生实例"""
    stu = Student.from_default()
    log.info(f"默认学生: {stu}")


def test_teacher_class():
    """教师类测试"""
    teacher = Teacher("张老师", "数学")
    log.info(f"教师: {teacher}")

    stu1 = Student("小明", 18, "高三", 3.5)
    stu2 = Student("小红", 17, "高三", 3.7)
    stu3 = Student("小刚", 18, "高三", 3.9)

    teacher.add_student(stu1)
    teacher.add_student(stu2)
    teacher.add_student(stu3)

    log.info(f"学生数量: {teacher.get_student_count()}")
    log.info(f"所有学生: {teacher.get_all_students_info()}")
    log.info(f"平均年龄: {teacher.get_average_age()}")


def test_function():
    """函数调用示例"""
    a_json = {"a": "张三", "b": "莉丝", "c": "王五"}
    a_list = ["张三", "赵六", "田七", "hello"]
    log.info(many_args(999, *a_list, **a_json))
    log.info(f"字典转列表: {dic_to_list(a_json)}")
    log.info(f"列表转字典: {list_to_dic(a_list)}")


def test_string_functions():
    """字符串处理函数测试"""
    s = "Hello, World!"
    log.info(f"reverse_string('{s}'): {reverse_string(s)}")
    log.info(f"hash_string('{s}'): {hash_string(s)}")


def test_math_functions():
    """数学计算函数测试"""
    log.info(f"factorial(5): {factorial(5)}")
    log.info(f"factorial(10): {factorial(10)}")
    log.info(f"fibonacci(10): {fibonacci(10)}")
    log.info(f"fibonacci(20): {fibonacci(20)}")


def test_array_functions():
    """数组处理函数测试"""
    arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    log.info(f"unique_sorted({arr}): {unique_sorted(arr)}")

    arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    log.info(f"chunk_array({arr}, 3): {chunk_array(arr, 3)}")

    nested = [[1, 2], [3, 4, 5], [6]]
    log.info(f"flatten_array({nested}): {flatten_array(nested)}")


def test_performance():
    """性能测试"""
    start = time.time()
    large_arr = generate_large_array(1_000_000)
    log.info(f"generate_large_array(1_000_000) 耗时: {time.time() - start:.4f}s")
    log.info(f"数组长度: {len(large_arr)}")


def test_instance():
    """类 实例调用示例"""
    stu = Student("张三", 9)
    log.info(f"实例属性: {student_info(stu)}")
    stu.age = 100
    log.info(f"通过实例方法修改属性之后: {student_info(stu)}")
    stu2 = Student("李四", 19)
    stu.set_other_age(stu2)
    log.info(f"通过实例方法修改属性之后: {student_info(stu2)}")


def test_error_handling():
    """错误处理测试"""
    stu = Student("测试", 10)

    try:
        stu.update_info(age=200)
    except ValidationError as e:
        log.info(f"catch ValidationError: {e}")


if __name__ == "__main__":
    log.info("=" * 60)
    log.info("测试类功能")
    log.info("=" * 60)
    test_class()

    log.info("=" * 60)
    log.info("测试从字典创建")
    log.info("=" * 60)
    test_class_from_dict()

    log.info("=" * 60)
    log.info("测试从默认创建")
    log.info("=" * 60)
    test_class_from_default()

    log.info("=" * 60)
    log.info("测试教师类")
    log.info("=" * 60)
    test_teacher_class()

    log.info("=" * 60)
    log.info("测试函数")
    log.info("=" * 60)
    test_function()

    log.info("=" * 60)
    log.info("测试字符串函数")
    log.info("=" * 60)
    test_string_functions()

    log.info("=" * 60)
    log.info("测试数学函数")
    log.info("=" * 60)
    test_math_functions()

    log.info("=" * 60)
    log.info("测试数组函数")
    log.info("=" * 60)
    test_array_functions()

    log.info("=" * 60)
    log.info("测试性能")
    log.info("=" * 60)
    test_performance()

    log.info("=" * 60)
    log.info("测试实例")
    log.info("=" * 60)
    test_instance()

    log.info("=" * 60)
    log.info("测试错误处理")
    log.info("=" * 60)
    test_error_handling()
