#ifndef __EX_RSTRING_H__
#define __EX_RSTRING_H__

#include <glib-object.h>

G_BEGIN_DECLS

#define EX_TYPE_RSTRING            (ex_rstring_get_type())

typedef struct _ExRString          ExRString;

GType       ex_rstring_get_type    (void);

ExRString * ex_rstring_new         (const gchar * s);
ExRString * ex_rstring_copy        (const ExRString * rstring);
void        ex_rstring_free        (ExRString * rstring);
gchar *     ex_rstring_get         (const ExRString * rstring);
void        ex_rstring_set         (ExRString *rstring, const gchar *s);

G_END_DECLS

#endif /* __EX_RSTRING_H__ */
