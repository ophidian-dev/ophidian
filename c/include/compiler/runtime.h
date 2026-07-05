#ifndef RUNTIME_H
#define RUNTIME_H

#include <stdint.h>
#include <stddef.h>

enum vm_ValueType {
    VT_INT   
};

typedef struct vm_Value {
    enum vm_ValueType type;
    union {
        int i;
    } as;
} vm_Value;

vm_Value vm_create_int_value(int i);

void vm_execute(uint8_t *bytecode, size_t bytecode_len, vm_Value *constants, size_t constant_len);

#endif // RUNTIME_H