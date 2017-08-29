#ifndef __EX_FOO_H__
#define __EX_FOO_H__

#include <glib-object.h>

G_BEGIN_DECLS

#define EX_TYPE_FOO            (ex_foo_get_type())
#define EX_FOO(obj)            (G_TYPE_CHECK_INSTANCE_CAST((obj),EX_TYPE_FOO,GstExFoo))
#define EX_IS_FOO(obj)         (G_TYPE_CHECK_INSTANCE_TYPE((obj),EX_TYPE_FOO))
#define EX_FOO_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST((klass) ,EX_TYPE_FOO,GstExFooClass))
#define EX_IS_FOO_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE((klass) ,EX_TYPE_FOO))
#define EX_FOO_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj) ,EX_TYPE_FOO,GstExFooClass))

typedef struct _ExFoo      ExFoo;
typedef struct _ExFooClass ExFooClass;

struct _ExFoo {
  GObject parent;
};

struct _ExFooClass {
  GObjectClass parent_class;
};

GType   ex_foo_get_type    (void);

ExFoo * ex_foo_new         (const gchar * name);

gint    ex_foo_increment   (ExFoo * foo, gint inc);
gint    ex_foo_get_counter (ExFoo * foo);
gchar * ex_foo_get_name    (ExFoo * foo);

G_END_DECLS

#endif /* __EX_FOO_H__ */
