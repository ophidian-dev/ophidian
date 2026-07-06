#include "compiler/runtime.h"

#include "compiler/vm.h"

vm_Value vm_create_int_value(int i) {
    vm_Value value;
    value.type = VT_INT;
    value.as.integer = i;
    return value;
}

void vm_execute(uint8_t *bytecode, size_t bytecode_len, vm_Value *constants, size_t constant_len) {
    struct VM vm;
    vm_init(&vm);

    vm_execute(bytecode, bytecode_len, constants, constant_len);

    vm_free(&vm);
}

