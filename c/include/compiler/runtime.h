#ifndef RUNTIME_H
#define RUNTIME_H

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

#endif // RUNTIME_H