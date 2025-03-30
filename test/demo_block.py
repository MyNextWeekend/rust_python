from log_utils import Logger
from rust_python import (
    ChildErrorA,
    Student,
    dic_to_list,
    list_to_dic,
    many_args,
    student_info,
)

log = Logger().get_logger()


def test_class():
    log.info(f"类 方法调用示例{'=' * 20}")
    stu = Student("里斯", 9)
    log.info(f"name:{stu.name},age:{stu.age}")
    b_list = [1, 2, 3, 4, 50, 10]
    stu.py_set_large_age(b_list)
    log.info(f"设置最大值之后: {stu}")

    log.info(f"获异常示例{'=' * 20}")
    try:
        stu.raise_exception(1)
    except ChildErrorA as e:
        log.info(f"catch except: {e}")


def test_function():
    log.info(f"函数调用示例{'=' * 20}")
    a_json = {"a": "张三", "b": "莉丝", "c": "王五"}
    a_list = ["张三", "赵六", "田七", "hello"]
    log.info(many_args(999, *a_list, **a_json))
    log.info(f"字典转列表: {dic_to_list(a_json)}")
    log.info(f"列表转字典: {list_to_dic(a_list)}")


def test_instance():
    log.info(f"类 实例 示例{'=' * 20}")
    stu = Student("张三", 9)
    log.info(f"实例属性: {student_info(stu)}")
    # stu.set_age(10)
    log.info(f"通过实例方法修改属性之后: {student_info(stu)}")


if __name__ == "__main__":
    test_class()
    test_function()
    test_instance()
