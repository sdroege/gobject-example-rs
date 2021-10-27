#include <glib.h>
#include <ex.h>

static void
on_incremented (ExFoo *foo, gint val, gint inc, void *user_data)
{
    g_print ("incremented to %d by %d\n", val, inc);
}

int
main (int argc, const char *argv[])
{
    char *str;
    g_autoptr(ExFoo) foo = NULL;
    g_autoptr(ExBar) bar = NULL;
    ExRString *s, *s2;
    ExSharedRString *ss, *ss2;
    gdouble number;

    foo = ex_foo_new ("foo's name");
    g_signal_connect (foo, "incremented", G_CALLBACK (on_incremented), NULL);

    str = ex_foo_get_name (foo);
    g_print ("foo name: %s\n", str);
    g_free (str);
    g_print ("foo inc 1: %d\n", ex_foo_increment (foo, 1));
    g_print ("foo inc 10: %d\n", ex_foo_increment (foo, 10));
    g_print ("foo counter: %d\n", ex_foo_get_counter (foo));

    bar = ex_bar_new ("bar's name");
    g_signal_connect (bar, "incremented", G_CALLBACK (on_incremented), NULL);

    str = ex_foo_get_name (EX_FOO (bar));
    g_print ("bar name: %s\n", str);
    g_free (str);
    g_print ("bar inc 1: %d\n", ex_foo_increment (EX_FOO (bar), 1));
    g_print ("bar inc 10: %d\n", ex_foo_increment (EX_FOO (bar), 10));
    g_print ("bar counter: %d\n", ex_foo_get_counter (EX_FOO (bar)));

    g_print ("bar number: %f\n", ex_bar_get_number (bar));
    g_object_get (bar, "number", &number, NULL);
    g_print ("bar number (property): %f\n", number);
    ex_bar_set_number (bar, 10.0);
    g_print ("bar number: %f\n", ex_bar_get_number (bar));
    g_object_get (bar, "number", &number, NULL);
    g_print ("bar number (property): %f\n", number);
    number = 20.0;
    g_object_set (bar, "number", number, NULL);
    g_print ("bar number: %f\n", ex_bar_get_number (bar));
    g_object_get (bar, "number", &number, NULL);
    g_print ("bar number (property): %f\n", number);

    s = ex_rstring_new ("something");

    str = ex_rstring_get (s);
    g_print ("rstring: %s\n", str);
    g_free (str);
    s2 = ex_rstring_copy (s);
    ex_rstring_set (s2, "something else");
    str = ex_rstring_get (s2);
    g_print ("rstring 2: %s\n", str);
    g_free (str);
    ex_rstring_free (s2);
    ex_rstring_free (s);

    ss = ex_shared_rstring_new ("something");
    str = ex_shared_rstring_get (ss);
    g_print ("shared rstring: %s\n", str);
    g_free (str);
    ss2 = ex_shared_rstring_ref (ss);
    str = ex_shared_rstring_get (ss2);
    g_print ("shared rstring 2: %s\n", str);
    g_free (str);
    ex_shared_rstring_unref (ss2);
    ex_shared_rstring_unref (ss);

    return 0;
}
