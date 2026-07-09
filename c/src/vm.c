#include "compiler/vm.h"

#include <stdlib.h>
#include <stdio.h>
#include <stddef.h>

#include "compiler/opcode.h"

typedef uint8_t Byte;

void stack_init(struct Stack *stack) {
    stack->size = 0;
    stack->capacity = 16;
    stack->data = malloc(sizeof(Value) * stack->capacity);
    if (!stack->data) {
        fprintf(stderr, "memory allocation failure\n");
        exit(1);
    }
}

void stack_push(struct Stack *stack, Value value) {
    if (stack->size >= stack->capacity) {
        size_t new_cap = stack->capacity * 2;
        Value *tmp = realloc(stack->data, sizeof(Value) * new_cap);
        if (!tmp) {
            free(stack->data);
            fprintf(stderr, "memory allocation failure\n");
            exit(1);
        } 
        stack->data = tmp;
        stack->capacity = new_cap;
    }
    stack->data[stack->size] = value;
    stack->size++;
}

Value stack_pop(struct Stack *stack) {
    if (stack->size < 1) {
        fprintf(stderr, "stack underflow\n");
        exit(1);
    }
    Value v = stack->data[stack->size - 1];
    stack->size--;
    return v;
}

void stack_free(struct Stack *stack) {
    free(stack->data);
    stack->data = NULL;
    stack->capacity = 0;
    stack->size = 0;
}

void vm_init(struct VM *vm) {
    struct Stack stack;
    stack_init(&stack);
    vm->stack = stack;
    vm->locals = calloc(LOCAL_MAX, sizeof(Value));
    if (!vm->locals) {
        fprintf(stderr, "memory allocation failure");
        exit(1);
    }
}

static Byte read_byte(struct VM *vm) {
    return *vm->ip++;
}

static void push(struct VM *vm, Value v) {
    stack_push(&vm->stack, v);
}

static Value pop(struct VM *vm) {
    return stack_pop(&vm->stack);
}

static Value value_from_int(int i) {
    return (Value) { .type = VT_INT, .as.integer = i };
}

static void clear_stack(struct Stack *stack) {
    stack->size = 0;
}

static uint32_t decode3_le(Byte b0, Byte b1, Byte b2) {
    return (uint32_t)b0
         | ((uint32_t)b1 << 8)
         | ((uint32_t)b2 << 16);
}

void vm_run(struct VM *vm, Byte *bytecode, size_t bytecode_len, Value *constants, size_t constant_len) {
    (void)constant_len;
    if (bytecode_len < 1) {
        return;
    } 

    clear_stack(&vm->stack);

    vm->ip = bytecode;
    vm->is_running = true;

    while (vm->is_running) {
        Byte opcode = read_byte(vm);

        switch (opcode) {
            case OP_HALT: {
                vm->is_running = false;
                break;
            }
            case OP_IADD: {
                Value b = pop(vm);
                Value a = pop(vm);
                Value v = value_from_int(a.as.integer + b.as.integer);
                push(vm, v);
                break;
            }
            case OP_ISUB: {
                Value b = pop(vm);
                Value a = pop(vm);
                Value v = value_from_int(a.as.integer - b.as.integer);
                push(vm, v);
                break;
            }
            case OP_IMUL: {
                Value b = pop(vm);
                Value a = pop(vm);
                Value v = value_from_int(a.as.integer * b.as.integer);
                push(vm, v);
                break;
            }
            case OP_IDIV: {
                Value b = pop(vm);
                Value a = pop(vm);
                Value v = value_from_int(a.as.integer / b.as.integer);
                push(vm, v);
                break;
            }
            case OP_INEGATE: {
                Value a = pop(vm);
                Value v = value_from_int(-a.as.integer);
                push(vm, v);
                break;
            }
            case OP_IPRINT: {
                Value v = pop(vm);
                printf("%d\n", v.as.integer);
                break;
            }
            case OP_POP: {
                pop(vm);
                break;
            }
            case OP_LOADCONST: {
                Byte b0 = read_byte(vm);
                Byte b1 = read_byte(vm);
                Byte b2 = read_byte(vm);
                uint32_t idx = decode3_le(b0, b1, b2);
                Value constant = constants[idx];
                push(vm, constant);
                break;
            }
            case OP_ILOAD_LOCAL: {
                Byte b0 = read_byte(vm);
                Byte b1 = read_byte(vm);
                Byte b2 = read_byte(vm);
                uint32_t idx = decode3_le(b0, b1, b2);
                Value v = vm->locals[idx];
                push(vm, v);
                break;
            }
            case OP_ISTORE_LOCAL: {
                Value store = pop(vm);
                Byte b0 = read_byte(vm);
                Byte b1 = read_byte(vm);
                Byte b2 = read_byte(vm);
                uint32_t idx = decode3_le(b0, b1, b2);
                vm->locals[idx] = store;
                break;
            }
            default: {
                fprintf(stderr, "unknown opcode: '%d'\n", (int)opcode);
                vm->is_running = false;
            }
        }
    }
} 

void vm_free(struct VM *vm) {
    stack_free(&vm->stack);
    free(vm->locals);
    vm->locals = NULL;
}
