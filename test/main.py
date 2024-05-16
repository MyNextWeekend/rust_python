import rust_python

a_json = {"a":1,"b":2,"c":3}
a_list=["张三",18,None,"hello"]


print(f'{"="*20} 函数调用 {"="*20}')
print(rust_python.many_args(999,*a_list,**a_json))
print(rust_python.dic_to_list(a_json))
print(rust_python.list_to_dic(a_list))


print(f'{"="*20} 对象调用 {"="*20}')
student= rust_python.Student()
print(student)