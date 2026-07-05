#include "vm.h"

#include <stdlib.h>
#include <stdio.h>
#include <stddef.h>

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
