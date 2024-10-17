#include "vm.h"
#include "object.h"
#include "memory.h"
#include "chunk.h"
#include "value.h"
#include <stdio.h>
#include <stdlib.h>
#include "debug.h"
#include "compile.h"
#include <stdarg.h>
#include <string.h>

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
interpret(VM* vm, char const* source){
  Chunk chunk;
  init_chunk(&chunk);
  
  if(!compile(source, &chunk)){
    free_chunk(&chunk);
    return INTERPRET_COMPILE_ERROR;
  }

  vm->chunk = &chunk;
  vm->ip = chunk.code;

  InterpretResult res = run(vm);

  free_chunk(&chunk);

  return res;
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

// Peek the last dist element in stack.
static Value
peek(VM* vm, int dist){
  return vm->sp[-1 - dist];
}

// Whether value is false in Lox
static bool
is_false(Value value){
  return IS_NIL(value) || (IS_BOOL(value) && !AS_BOOL(value));
}

static void
concatenate(VM* vm){
  ObjString* rhs = AS_STRING(pop(vm));
  ObjString* lhs = AS_STRING(pop(vm));

  int length = lhs->length + rhs->length;
  char* chars = ALLOCATE(char, length + 1);
  memcpy(chars, lhs->chars, lhs->length);
  memcpy(chars + lhs->length, rhs->chars, rhs->length);
  chars[length] = '\0';

  ObjString* res = take_string(chars, length);
  push(vm, OBJ_VAL(res));
}

// Reset the %rsp.
static void 
reset_stack(VM* vm){
  vm->sp = vm->stack;
}

// Raise RuntimeError, reset the stack.
static void
runtime_error(VM* vm, char const* format, ...){
  va_list args;
  va_start(args, format);
  vfprintf(stderr, format, args);
  va_end(args);

  fputc('\n', stderr);
  size_t instruction = vm->ip - vm->chunk->code - 1;
  int line = vm->chunk->lines[instruction];
  fprintf(stderr, "[line %d]", line);
  reset_stack(vm);
}

static InterpretResult run(VM* vm){
  static void* label[] = {
    &&ins_return,
    &&ins_constant,
    &&ins_nil,
    &&ins_true,
    &&ins_false,
    &&ins_neg,
    &&ins_add,
    &&ins_sub,
    &&ins_mul,
    &&ins_div,
    &&ins_not,
    &&ins_equal,
    &&ins_greater,
    &&ins_less,
  };
#define READ_BYTE() (*vm->ip++)
#define READ_CONSTANT() (vm->chunk->constants.values[READ_BYTE()])
#define BINARY(type, op) \
  do{ \
    if(!IS_NUMBER(peek(vm, 0)) || !IS_NUMBER(peek(vm, 1))) { \
      runtime_error(vm, "oprands must be numbers."); \
      return INTERPRET_RUNTIME_ERROR; \
    } \
    double rhs = AS_NUMBER(pop(vm)); \
    double lhs = AS_NUMBER(pop(vm)); \
    push(vm, type(lhs op rhs)); \
  }while(0)

#define DISPATCH() goto *label[READ_BYTE()]

dispatch:
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

  DISPATCH();

ins_return:
  print_value(pop(vm));
  printf("\n");
  return INTERPRET_OK;

ins_constant:
  push(vm, READ_CONSTANT());
  goto dispatch;

ins_nil:
  push(vm, NIL_VAL);
  goto dispatch;

ins_true:
  push(vm, BOOL_VAL(true));
  goto dispatch;

ins_false:
  push(vm, BOOL_VAL(false));
  goto dispatch;

ins_neg:
  if(!IS_NUMBER(peek(vm, 0))){
    runtime_error(vm, "oprand of '-' must be a number.");
    return INTERPRET_RUNTIME_ERROR;
  }
  push(vm, NUMBER_VAL(-AS_NUMBER(pop(vm))));
  goto dispatch;

ins_add:
  if(IS_STRING(peek(vm, 0)) && IS_STRING(peek(vm, 1))){
    concatenate(vm);
  }else if(IS_NUMBER(peek(vm, 0)) && IS_NUMBER(peek(vm, 1))){
    double rhs = AS_NUMBER(pop(vm));
    double lhs = AS_NUMBER(pop(vm));
    push(vm, NUMBER_VAL(lhs + rhs));
  }else{
    runtime_error(vm, "operands must be numbers or strings.");
    return INTERPRET_RUNTIME_ERROR;
  }
  goto dispatch;

ins_sub:
  BINARY(NUMBER_VAL, -);
  goto dispatch;

ins_mul:
  BINARY(NUMBER_VAL, *);
  goto dispatch;

ins_div:
  BINARY(NUMBER_VAL, /);
  goto dispatch;

ins_not:
  push(vm, BOOL_VAL(is_false(pop(vm))));
  goto dispatch;

ins_equal:
  do{
    Value b = pop(vm);
    Value a = pop(vm);
    push(vm, BOOL_VAL(values_equal(a, b)));
  }while(0);
  goto dispatch;

ins_greater:
  BINARY(BOOL_VAL, >);
  goto dispatch;

ins_less:
  BINARY(BOOL_VAL, <);
  goto dispatch;

#undef READ_BYTE
#undef READ_CONSTANT
#undef BINARY
}