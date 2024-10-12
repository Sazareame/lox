#include "vm.h"
#include "chunk.h"
#include "common.h"
#include "value.h"
#include <stdio.h>
#include <stdlib.h>

static InterpretResult run(VM*);

void
init_VM(VM** vm){
  *vm = (VM*)malloc(sizeof(VM));
}

void
free_VM(VM *vm){
  if(vm) free(vm);
}

InterpretResult
interpret(VM *vm, Chunk *chunk){
  vm->chunk = chunk;
  vm->ip = vm->chunk->code;
  return run(vm);
}

static InterpretResult run(VM* vm){
#define READ_BYTE() (*vm->ip++)
#define READ_CONSTANT() (vm->chunk->constants.values[READ_BYTE()])

  while(1){
    uint8_t ins;
    switch(ins = READ_BYTE()){
      case OP_RETURN: {
        return INTERPRET_OK;
      }
      case OP_CONSTANT: {
        Value constant = READ_CONSTANT();
        print_value(constant);
        printf("\n");
        break;
      }
    }
  }

#undef READ_BYTE
#undef READ_CONSTANT
}