#ifndef LOX_OBJ_H
#define LOX_OBJ_H

#include "value.h"

#define OBJ_TYPE(value) (AS_OBJ(value)->type)
#define IS_STRING(value) is_object_type(value, OBJ_STRING)
// cast a OBj value to ObjString
#define AS_STRING(value) ((ObjString*)AS_OBJ(value))
// get the underlying char* from a Obj value
#define AS_CSTRING(value) (((ObjString*)AS_OBJ(value))->chars)

typedef enum{
  OBJ_STRING,
}ObjType;

struct Obj{
  ObjType type;
};

struct ObjString{
  Obj obj;
  int length;
  char* chars;
};

static inline bool
is_object_type(Value value, ObjType type){
  return IS_OBJ(value) && AS_OBJ(value)->type == type;
}

// Create an ObjString from source, by copying.
ObjString* copy_string(char const* chars, int length);
// Create an ObjString from other Lox String, by taking ownership.
ObjString* take_string(char* chars, int length);
void print_obj(Value value);

#endif