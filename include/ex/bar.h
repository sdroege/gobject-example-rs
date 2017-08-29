#ifndef __EX_BAR_H__
#define __EX_BAR_H__

#include <glib-object.h>

G_BEGIN_DECLS

#define EX_TYPE_BAR            (ex_bar_get_type())
#define EX_BAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST((obj),EX_TYPE_BAR,GstExBar))
#define EX_IS_BAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE((obj),EX_TYPE_BAR))
#define EX_BAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST((klass) ,EX_TYPE_BAR,GstExBarClass))
#define EX_IS_BAR_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE((klass) ,EX_TYPE_BAR))
#define EX_BAR_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj) ,EX_TYPE_BAR,GstExBarClass))

typedef struct _ExBar      ExBar;
typedef struct _ExBarClass ExBarClass;

struct _ExBar {
  GObject parent;
};

struct _ExBarClass {
  GObjectClass parent_class;
};

GType   ex_bar_get_type    (void);

ExBar * ex_bar_new         (const gchar * name);

gdouble ex_bar_get_number  (ExBar *bar);
void    ex_bar_set_number  (ExBar *bar, gdouble num);

G_END_DECLS

#endif /* __EX_BAR_H__ */
