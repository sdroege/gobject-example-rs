#ifndef __EX_FLAGS_H__
#define __EX_FLAGS_H__

#include <glib-object.h>

G_BEGIN_DECLS

typedef enum ExFlags
{
  EX_FLAGS_SOME = 1 << 0,
  EX_FLAGS_ZING = 1 << 1,
  EX_FLAGS_BONG = 1 << 2,
} ExFlags;

#define EX_TYPE_FLAGS            (ex_flags_get_type())

GType   ex_flags_get_type       (void);

G_END_DECLS

#endif /* __EX_FLAGS_H__ */
