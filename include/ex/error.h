#ifndef __ERROR_H_
#define __ERROR_H_

#include <glib-object.h>

G_BEGIN_DECLS

typedef enum ExError
{
  EX_ERROR_INVALID_ARGUMENT,
  EX_ERROR_FAILED,
} ExError;

#define EX_ERROR           (ex_error_quark())

GQuark ex_error_quark      (void);

G_END_DECLS

#endif /* __ERROR_H_ */
