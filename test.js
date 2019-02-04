#!/usr/bin/gjs

const Lang = imports.lang;
const Ex = imports.gi.Ex;

let foo = new Ex.Foo({name: "foo's name"});
foo.connect("incremented", function(obj, val, inc) {
    print("incremented to " + val + " by " + inc);
});

print("foo name: " + foo.get_name());
print("foo inc 1: " + foo.increment(1));
print("foo inc 10: " + foo.increment(10));
print("foo counter: " + foo.get_counter());

let bar = new Ex.Bar({name: "bar's name"});
bar.connect("incremented", function(obj, val, inc) {
    print("incremented to " + val + " by " + inc);
});

print("bar name: " + bar.get_name());
print("bar inc 1: " + bar.increment(1));
print("bar inc 10: " + bar.increment(10));
print("bar counter: " + bar.get_counter());

print("bar number: " + bar.get_number());
print("bar number (property): " + bar["number"]);
bar.set_number(10.0)
print("bar number: " + bar.get_number());
print("bar number (property): " + bar["number"]);
bar["number"] = 20.0;
print("bar number: " + bar.get_number());
print("bar number (property): " + bar["number"]);

let s = new Ex.RString("something");
print("rstring: " + s.get());
let s2 = s.copy();
s2.set("something else");
print("rstring2: " + s2.get());

let t = new Ex.SharedRString("something");
print("shared rstring: " + t.get());
let t2 = t.ref();
print("shared rstring2: " + t2.get());


