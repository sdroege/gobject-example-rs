#ifndef __EX_NAMEABLE_H__
#define __EX_NAMEABLE_H__

#include <glib-object.h>

G_BEGIN_DECLS

#define EX_TYPE_NAMEABLE                (ex_nameable_get_type())
#define EX_NAMEABLE(obj)                (G_TYPE_CHECK_INSTANCE_CAST((obj)    ,EX_TYPE_NAMEABLE,ExNameable))
#define EX_IS_NAMEABLE(obj)             (G_TYPE_CHECK_INSTANCE_TYPE((obj)    ,EX_TYPE_NAMEABLE))
#define EX_NAMEABLE_GET_INTERFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE((obj) ,EX_TYPE_NAMEABLE,ExNameableInterface))

typedef struct _ExNameable      ExNameable;
typedef struct _ExNameableInterface ExNameableInterface;

struct _ExNameableInterface {
  GTypeInterface parent_interface;

  gchar * (*get_name) (ExNameable * nameable);
};

GType   ex_nameable_get_type    (void);

gchar * ex_nameable_get_name    (ExNameable * nameable);

G_END_DECLS

#endif /* __EX_NAMEABLE_H__ */
