#ifndef RUNTIME_H
#define RUNTIME_H

enum ValueType {
    VT_INT   
};

typedef struct Value {
    enum ValueType type;
    union {
        int i;
    } as;
} Value;

#endif // RUNTIME_H