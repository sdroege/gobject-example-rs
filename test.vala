using Ex;

void on_incremented (int val, int inc) {
	stdout.printf ("incremented to %d by %d\n", val, inc);
}

public int main (string[] args) {
	var foo = new Ex.Foo ("foo's name");
	foo.incremented.connect (on_incremented);

	stdout.printf ("foo name: %s\n", foo.get_name ());
	stdout.printf ("foo inc 1: %d\n", foo.increment (1));
	stdout.printf ("foo inc 10: %d\n", foo.increment (10));
	stdout.printf ("foo counter: %d\n", foo.get_counter ());

	var bar = new Ex.Bar ("bar's name");
	bar.incremented.connect (on_incremented);

	stdout.printf ("bar name: %s\n", bar.get_name ());
	stdout.printf ("bar inc 1: %d\n", bar.increment (1));
	stdout.printf ("bar inc 10: %d\n", bar.increment (10));
	stdout.printf ("bar counter: %d\n", bar.get_counter ());

	stdout.printf ("bar number: %f\n", bar.get_number ());
	stdout.printf ("bar number (property): %f\n", bar.number);
	bar.set_number (10.0);
	stdout.printf ("bar number: %f\n", bar.get_number ());
	stdout.printf ("bar number (property): %f\n", bar.number);
	bar.number = 20.0;
	stdout.printf ("bar number: %f\n", bar.get_number ());
	stdout.printf ("bar number (property): %f\n", bar.number);

	var s = new Ex.RString ("something");
	stdout.printf ("rstring: %s\n", s.get ());
	var s2 = s.copy ();
	s2.set ("something else");
	stdout.printf ("rstring 2: %s\n", s2.get ());

	var ss = new Ex.SharedRString ("something");
	stdout.printf ("shared rstring: %s\n", ss.get ());
	var ss2 = ss.ref ();
	stdout.printf ("shared rstring 2: %s\n", ss2.get ());

	return 0;
}
