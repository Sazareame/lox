#include <stdlib.h>
#include "memory.h"

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