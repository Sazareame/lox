#include "vm.h"
#include "chunk.h"
#include "common.h"
#include "value.h"
#include <stdio.h>
#include <stdlib.h>
#include "debug.h"

static InterpretResult run(VM*);
static void reset_stack(VM*);

void
init_VM(VM** vm){
  *vm = (VM*)malloc(sizeof(VM));
  reset_stack(*vm);
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

void
push(VM* vm, Value value){
  *vm->sp++ = value;
}

Value
pop(VM* vm){
  // sp points to the `next` empty postion in stack.
  return *--vm->sp;
}


// Reset the %rsp.
static void 
reset_stack(VM* vm){
  vm->sp = vm->stack;
}

static InterpretResult run(VM* vm){
#define READ_BYTE() (*vm->ip++)
#define READ_CONSTANT() (vm->chunk->constants.values[READ_BYTE()])

  while(1){
#ifdef DEBUG_TRACE_EXECUTION
    disassemble_instruction(vm->chunk, (int)(vm->ip - vm->chunk->code));

    printf("== stack ==\n");
    for(Value* p = vm->stack; p < vm->sp; ++p){
      printf("[ ");
      print_value(*p);
      printf(" ]");
    }
    printf("\n");
#endif
    uint8_t ins;
    switch(ins = READ_BYTE()){
      case OP_RETURN: {
        print_value(pop(vm));
        printf("\n");
        return INTERPRET_OK;
      }
      case OP_CONSTANT: {
        Value constant = READ_CONSTANT();
        push(vm, constant);
        break;
      }
    }
  }

#undef READ_BYTE
#undef READ_CONSTANT
}