#ifndef __EX_SHARED_RSTRING_H__
#define __EX_SHARED_RSTRING_H__

#include <glib-object.h>

G_BEGIN_DECLS

#define EX_TYPE_SHARED_RSTRING           (ex_shared_rstring_get_type())

typedef struct _ExSharedRString          ExSharedRString;

GType             ex_shared_rstring_get_type    (void);

ExSharedRString * ex_shared_rstring_new         (const gchar * s);
ExSharedRString * ex_shared_rstring_ref         (ExSharedRString * shared_rstring);
void              ex_shared_rstring_unref       (ExSharedRString * shared_rstring);
gchar *           ex_shared_rstring_get         (ExSharedRString * shared_rstring);

G_END_DECLS

#endif /* __EX_SHARED_RSTRING_H__ */
