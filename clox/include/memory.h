#ifndef LOX_MEMORY_H_
#define LOX_MEMORY_H_

#include "common.h"
#include "object.h"

#define GROW_CAPACITY(capacity) \
  ((capacity) < 8 ? 8 : (capacity) * 2)

#define GROW_ARRAY(type, ptr, old, new) \
  (type*)reallocate(ptr, sizeof(type) * (old), sizeof(type) * (new))

#define FREE_ARRAY(type, ptr, count) \
  reallocate(ptr, sizeof(type) * count, 0)

#define ALLOCATE(type, count) \
  (type*)reallocate(NULL, 0, sizeof(type) * (count))

#define FREE(type, ptr) reallocate(ptr, sizeof(type), 0)

void* reallocate(void* ptr, size_t old, size_t new);
void free_objects(Obj* obj);

#endif