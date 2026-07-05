#include "compiler/runtime.h"

Value create_int_value(int i) {
    Value value;
    value.type = VT_INT;
    value.as.i = i;
    return value;
}
