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
    """类、方法调用示例"""
    stu = Student("里斯", 9)
    # 有get_all注解的时候才可以获取属性的值
    log.info(f"name:{stu.name},age:{stu.age}")
    age_list = [1, 2, 3, 4, 50, 10]
    stu.py_set_large_age(age_list)

    try:
        stu.raise_exception(2)
    except ChildErrorA as e:
        log.info(f"catch ChildErrorA: {e}")
    except Exception as e:
        log.error(f"catch Exception: {e}")


def test_function():
    """函数调用示例"""
    a_json = {"a": "张三", "b": "莉丝", "c": "王五"}
    a_list = ["张三", "赵六", "田七", "hello"]
    log.info(many_args(999, *a_list, **a_json))
    log.info(f"字典转列表: {dic_to_list(a_json)}")
    log.info(f"列表转字典: {list_to_dic(a_list)}")


def test_instance():
    """类 实例调用示例"""
    stu = Student("张三", 9)
    log.info(f"实例属性: {student_info(stu)}")
    # 有set_all注解的时候才可以设置属性的值
    stu.age = 100
    log.info(f"通过实例方法修改属性之后: {student_info(stu)}")


if __name__ == "__main__":
    test_class()
    test_function()
    test_instance()
