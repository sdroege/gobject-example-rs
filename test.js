#!/usr/bin/gjs

const Lang = imports.lang;
const Ex = imports.gi.Ex;

let foo = new Ex.Foo();
print("foo name: " + foo.get_name());
print("foo inc 1: " + foo.increment(1));
print("foo inc 10: " + foo.increment(10));
print("foo counter: " + foo.get_counter());

let bar = new Ex.Bar();
print("bar name: " + bar.get_name());
print("bar inc 1: " + bar.increment(1));
print("bar inc 10: " + bar.increment(10));
print("bar counter: " + bar.get_counter());
