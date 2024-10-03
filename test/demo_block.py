from rust_python import (many_args, dic_to_list, list_to_dic,
                         student_set_age, student_info,
                         Student,
                         ChildErrorA)

a_json = {"a": "张三", "b": "莉丝", "c": "王五"}
a_list = ["张三", "赵六", "田七", "hello"]

print(f'{"=" * 20} 函数调用 {"=" * 20}')
print(many_args(999, *a_list, **a_json))
print(dic_to_list(a_json))
print(list_to_dic(a_list))

print(f'{"=" * 20} 对象调用 {"=" * 20}')
stu = Student("里斯", 9)
print(stu)
print(f"name:{stu.name},age:{stu.age}")

try:
    stu.raise_exception(1)
except ChildErrorA as e:
    print(f"catch except: {e}")

b_list = [1, 2, 3, 4, 50, 10]
stu.py_set_large_age(b_list)
print(f"设置最大值之后: {stu}")

# 传入 类实例 作为型参
student_set_age(stu, 15)
print(f"通过可变引用修改属性之后: {student_info(stu)}")
