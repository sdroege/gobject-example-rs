#! /usr/bin/python3

import gi
gi.require_version("Ex", "0.1")
from gi.repository import Ex

def on_incremented(obj, val, inc):
    print("incremented to {} by {}".format(val, inc))

foo = Ex.Foo.new("foo's name")
foo.connect("incremented", on_incremented)

print("foo name: " + str(foo.get_name()))
print("foo inc 1: " + str(foo.increment(1)))
print("foo inc 10: " + str(foo.increment(10)))
print("foo counter: " + str(foo.get_counter()))

bar = Ex.Bar.new("bar's name")
bar.connect("incremented", on_incremented)

print("bar name: " + str(bar.get_name()))
print("bar inc 1: " + str(bar.increment(1)))
print("bar inc 10: " + str(bar.increment(10)))
print("bar counter: " + str(bar.get_counter()))

print("bar number: " + str(bar.get_number()))
print("bar number (property): " + str(bar.get_property("number")))
bar.set_number(10.0)
print("bar number: " + str(bar.get_number()))
print("bar number (property): " + str(bar.get_property("number")))
bar.set_property("number", 20.0)
print("bar number: " + str(bar.get_number()))
print("bar number (property): " + str(bar.get_property("number")))
