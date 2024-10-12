#ifndef LOX_VM_H_
#define LOX_VM_H_

#include "chunk.h"

typedef struct{
  Chunk* chunk;
  // aka. IP register, which stores the address of the next instruction.
  uint8_t* ip;
}VM;

typedef enum{
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
}InterpretResult;

void init_VM(VM** vm);
void free_VM(VM* vm);

InterpretResult interpret(VM* vm, Chunk* chunk);

#endif