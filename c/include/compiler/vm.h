#ifndef VM_H
#define VM_H

#include <stddef.h>

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
};

#endif // VM_H
