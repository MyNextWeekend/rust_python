# -*- coding: utf-8 -*-
# @Time    : 2021/4/16 21:58
# @Author  : MyNextWeekend
import functools
import logging
import pathlib
from logging.handlers import TimedRotatingFileHandler

BASE_PATH = pathlib.Path(__file__).parent.parent


def singleton(cls):
    """
    将一个类作为单例
    来自 https://wiki.python.org/moin/PythonDecoratorLibrary#Singleton
    """

    cls.__new_original__ = cls.__new__

    @functools.wraps(cls.__new__)
    def singleton_new(cls, *args, **kw):
        it = cls.__dict__.get("__it__")
        if it is not None:
            return it

        cls.__it__ = it = cls.__new_original__(cls, *args, **kw)
        it.__init_original__(*args, **kw)
        return it

    cls.__new__ = singleton_new
    cls.__init_original__ = cls.__init__
    cls.__init__ = object.__init__

    return cls


@singleton
class Logger:
    def __init__(self):
        # 如果已经初始化了就不再执行，避免重复添加handle
        self.Flag = False
        self.fmt_str = (
            "%(asctime)s【%(levelname)s】-%(filename)s[%(lineno)d]: %(message)s"
        )
        self.logger = logging.getLogger()
        self.logger.setLevel(logging.DEBUG)
        # 添加handle
        self.logger.addHandler(self.console_handle())
        self.logger.addHandler(self.file_handle())

    def file_handle(self):
        """日志文件的handle"""

        filename = BASE_PATH.joinpath("logs", "rust_python.log")
        file_handle = TimedRotatingFileHandler(
            filename, when="midnight", backupCount=5, encoding="utf-8"
        )
        file_handle.setLevel(logging.INFO)
        fmt = logging.Formatter(self.fmt_str)
        file_handle.setFormatter(fmt)
        return file_handle

    def console_handle(self):
        """控制台日志的handle"""
        console_handle = logging.StreamHandler()
        console_handle.setLevel(logging.DEBUG)
        fmt = logging.Formatter(self.fmt_str)
        console_handle.setFormatter(fmt)
        return console_handle

    def get_logger(self):
        return self.logger


if __name__ == "__main__":
    print(f"{BASE_PATH=}")
