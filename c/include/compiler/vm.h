#ifndef VM_H
#define VM_H

#include <stddef.h>
#include <stdint.h>

#include "runtime.h"

typedef vm_Value Value;
typedef enum vm_ValueType ValueType;


struct Stack {
    Value *data;
    size_t size;
    size_t capacity;
};

void stack_init(struct Stack *stack);
void stack_push(struct Stack *stack, Value value);
Value stack_pop(struct Stack *stack);
void stack_free(struct Stack *stack);

struct VM {
    struct Stack stack;
    uint8_t *ip;
};

void vm_init(struct VM *vm);
void vm_run(struct VM *vm, uint8_t *bytecode, size_t bytecode_len, Value *constants, size_t constant_len);
void vm_free(struct VM *vm);

#endif // VM_H
