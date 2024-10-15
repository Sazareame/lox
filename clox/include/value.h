#ifndef LOX_VALUE_H_
#define LOX_VALUE_H_

#include "common.h"


// cast a C type into Lox type
#define BOOL_VAL(value) ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})

// cast a Lox type into C type
#define AS_BOOL(value) ((value).as.boolean)
#define AS_NUMBER(value) ((value).as.number)

// check the type under Value Union
#define IS_BOOL(value) ((value).type == VAL_BOOL)
#define IS_NIL(value) ((value).type == VAL_NIL)
#define IS_NUMBER(value) ((value).type == VAL_NUMBER)

typedef enum{
  VAL_BOOL,
  VAL_NIL,
  VAL_NUMBER,
}ValueType;

typedef struct{
  ValueType type;
  union{
    bool boolean;
    double number;
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
void print_value(Value val);

#endif