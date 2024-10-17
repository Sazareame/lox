#include <stdlib.h>
#include "memory.h"

static void free_object(Obj* obj);

void*
reallocate(void *ptr, size_t old, size_t new){
  if(!new){
    if(ptr) free(ptr);
    return 0;
  }
  void* ret = realloc(ptr, new);
  if(!ret) exit(1);
  return ret;
}

void
free_objects(Obj* obj){
  while(obj){
    Obj* next = obj->next;
    free_object(obj);
    obj = next;
  }
}

static void
free_object(Obj *obj){
  switch(obj->type){
    case OBJ_STRING: {
      ObjString* string = (ObjString*)obj;
      FREE_ARRAY(char, string->chars, string->length + 1);
      FREE(ObjString, obj);
      break;
    }
    default: return;
  }
}