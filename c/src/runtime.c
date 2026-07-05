#include "compiler/runtime.h"

vm_Value vm_create_int_value(int i) {
    vm_Value value;
    value.type = VT_INT;
    value.as.i = i;
    return value;
}
