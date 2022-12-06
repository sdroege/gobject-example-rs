#ifndef __EX_FOO_H__
#define __EX_FOO_H__

#include <glib-object.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define EX_TYPE_FOO            (ex_foo_get_type())
#define EX_FOO(obj)            (G_TYPE_CHECK_INSTANCE_CAST((obj),EX_TYPE_FOO,ExFoo))
#define EX_IS_FOO(obj)         (G_TYPE_CHECK_INSTANCE_TYPE((obj),EX_TYPE_FOO))
#define EX_FOO_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST((klass) ,EX_TYPE_FOO,ExFooClass))
#define EX_IS_FOO_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE((klass) ,EX_TYPE_FOO))
#define EX_FOO_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj) ,EX_TYPE_FOO,ExFooClass))

typedef struct _ExFoo      ExFoo;
typedef struct _ExFooClass ExFooClass;

struct _ExFoo {
  GObject parent;
};

struct _ExFooClass {
  GObjectClass parent_class;

  gint (*increment) (ExFoo * foo, gint inc);
  void (*incremented) (ExFoo * foo, gint val, gint inc);
};

GType   ex_foo_get_type    (void);

ExFoo * ex_foo_new         (const gchar * name);

gint    ex_foo_increment   (ExFoo * foo, gint inc);
gint    ex_foo_get_counter (ExFoo * foo);
gchar * ex_foo_get_name    (ExFoo * foo);

void        ex_foo_check_async  (ExFoo * foo, GCancellable *cancellable, GAsyncReadyCallback callback, gpointer user_data);
gboolean    ex_foo_check_finish (ExFoo * foo, GAsyncResult *res, GError **error);

G_END_DECLS

#endif /* __EX_FOO_H__ */
