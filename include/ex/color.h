#ifndef __EX_COLOR_H__
#define __EX_COLOR_H__

#include <glib-object.h>

G_BEGIN_DECLS

typedef enum ExColor
{
  EX_COLOR_RED,
  EX_COLOR_GREEN,
  EX_COLOR_BLUE,
} ExColor;

#define EX_TYPE_COLOR            (ex_color_get_type())

GType   ex_color_get_type       (void);

G_END_DECLS

#endif /* __EX_COLOR_H__ */
