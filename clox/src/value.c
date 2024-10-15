#include "value.h"
#include "memory.h"
#include <stdio.h>

void
init_value_array(ValueArray *arr){
  arr->values = NULL;
  arr->capacity = 0;
  arr->count = 0;
}

void
write_value_array(ValueArray *arr, Value value){
  if(arr->capacity < arr->count + 1){
    int old = arr->capacity;
    arr->capacity = GROW_CAPACITY(old);
    arr->values = GROW_ARRAY(Value, arr->values, old, arr->capacity);
  }
  arr->values[arr->count++] = value;
}

void
free_value_array(ValueArray *arr){
  FREE_ARRAY(Value, arr->values, arr->capacity);
  init_value_array(arr);
}

void
print_value(Value value){
  switch(value.type){
    case VAL_BOOL: printf(AS_BOOL(value) ? "true" : "false"); break;
    case VAL_NIL: printf("nil"); break;
    case VAL_NUMBER: printf("%g", AS_NUMBER(value)); break;
  }
}