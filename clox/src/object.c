#include "object.h"
#include <string.h>
#include "memory.h"
#include <stdio.h>
#include "vm.h"

#define ALLOCATE_OBJ(type, obj_type) \
  (type*)allocate_obj(sizeof(type), obj_type)

static Obj*
allocate_obj(VM* vm, size_t size, ObjType type){
  Obj* object = (Obj*)reallocate(NULL, 0, size);
  object->type = type;

  object->next = vm->objects;

  return object;
}

static ObjString*
allocate_string(char* chars, int length){
  ObjString* string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
  string->length = length;
  string->chars = chars;
  return string;
}

ObjString*
copy_string(char const* chars, int length){
  char* heap_chars = ALLOCATE(char, length + 1);
  memcpy(heap_chars, chars, length);
  heap_chars[length] = '\0';
  return allocate_string(heap_chars, length);
}

ObjString*
take_string(char* chars, int length){
  return allocate_string(chars, length);
}

void
print_obj(Value value){
  switch (OBJ_TYPE(value)){
    case OBJ_STRING: printf("%s", AS_CSTRING(value)); break;
  }
}