#! /usr/bin/python3

import gi
gi.require_version("Ex", "0.1")
from gi.repository import Ex

foo = Ex.Foo.new("foo's name")
print("foo name: " + str(foo.get_name()))
print("foo inc 1: " + str(foo.increment(1)))
print("foo inc 10: " + str(foo.increment(10)))
print("foo counter: " + str(foo.get_counter()))

bar = Ex.Bar.new("bar's name")
print("bar name: " + str(bar.get_name()))
print("bar inc 1: " + str(bar.increment(1)))
print("bar inc 10: " + str(bar.increment(10)))
print("bar counter: " + str(bar.get_counter()))
