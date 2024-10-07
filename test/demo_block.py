from rust_python import (many_args, dic_to_list, list_to_dic,
                         student_set_age, student_info,
                         Student,
                         ChildErrorA)
from log_utils import SingletonLog

log = SingletonLog().get_logger()

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

log.info(f"函数调用示例{'=' * 20}")
a_json = {"a": "张三", "b": "莉丝", "c": "王五"}
a_list = ["张三", "赵六", "田七", "hello"]
log.info(many_args(999, *a_list, **a_json))
log.info(f"字典转列表: {dic_to_list(a_json)}")
log.info(f"列表转字典: {list_to_dic(a_list)}")

log.info(f"函数 操作 类 实例 示例{'=' * 20}")
student_set_age(stu, 15)
log.info(f"通过可变引用修改属性之后: {student_info(stu)}")
