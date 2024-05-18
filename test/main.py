import rust_python
from rust_python import MyError,ChildErrorA

a_json = {"a":"张三","b":"莉丝","c":"王五"}
a_list=["张三","赵六","田七","hello"]


print(f'{"="*20} 函数调用 {"="*20}')
print(rust_python.many_args(999,*a_list,**a_json))
print(rust_python.dic_to_list(a_json))
print(rust_python.list_to_dic(a_list))


print(f'{"="*20} 对象调用 {"="*20}')
student= rust_python.Student("里斯",9)
print(student)
print(student.age)
print(student.name)

try:
    student.raise_exception(1)
except ChildErrorA as e:
    print(e)

b_list = [1,2,3,4,50,10]
print(student.py_set_lages_age(b_list))
print(student)