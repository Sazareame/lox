#ifndef LOX_VM_H_
#define LOX_VM_H_

#include "chunk.h"
#include "object.h"

// Max stack length
#define STACK_MAX 255

typedef struct{
  Chunk* chunk;
  // aka. IP register, which stores the address of the next instruction.
  uint8_t* ip;
  // aka. SP register, which point to the top of stack.
  Value* sp;
  // Link list head points to objects that are allocated on the heap.  
  // For GC.
  Obj* objects;
  // The VM stack.
  Value stack[STACK_MAX];
}VM;

typedef enum{
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
}InterpretResult;

void init_VM(VM** vm);
void free_VM(VM* vm);

InterpretResult interpret(VM* vm, char const* source);
void push(VM* vm, Value value);
Value pop(VM* vm);
// Whether two Lox Value is equal.
bool values_equal(Value a, Value b);

#endif