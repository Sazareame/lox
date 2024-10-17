#ifndef LOX_VALUE_H_
#define LOX_VALUE_H_

#include "common.h"


// cast a C type into Lox type
#define BOOL_VAL(value) ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})
#define OBJ_VAL(value) ((Value){VAL_OBJ, {.obj = (Obj*)value}})

// cast a Lox type into C type
#define AS_BOOL(value) ((value).as.boolean)
#define AS_NUMBER(value) ((value).as.number)
#define AS_OBJ(value) ((value).as.obj)

// check the type under Value Union
#define IS_BOOL(value) ((value).type == VAL_BOOL)
#define IS_NIL(value) ((value).type == VAL_NIL)
#define IS_NUMBER(value) ((value).type == VAL_NUMBER)
#define IS_OBJ(value) ((value).type == VAL_OBJ)

typedef enum{
  VAL_BOOL,
  VAL_NIL,
  VAL_NUMBER,
  VAL_OBJ,
}ValueType;

typedef struct Obj Obj;
typedef struct ObjString ObjString;

typedef struct{
  ValueType type;
  union{
    bool boolean;
    double number;
    Obj* obj;
  }as;
}Value;

typedef struct{
  int capacity;
  int count;
  Value* values;
}ValueArray;

void init_value_array(ValueArray* arr);
void write_value_array(ValueArray* arr, Value value);
void free_value_array(ValueArray* arr);
bool values_equal(Value a, Value b);
void print_value(Value val);

#endif