#ifndef CHUNK_H_
#define CHUNK_H_

#include "common.h"
#include "value.h"

typedef enum{
  OP_RETURN,
  OP_CONSTANT,
  OP_NIL,
  OP_TRUE,
  OP_FALSE,
  OP_NEG,
  OP_ADD,
  OP_SUB,
  OP_MUL,
  OP_DIV,
  OP_NOT,
} OpCode;

typedef struct{
  int count;
  int capacity;
  // A parallel array associated with code array used to represent the
  // line number the correspoding opcode comes from.
  int* lines;
  uint8_t* code;
  ValueArray constants;
}Chunk;

void init_chunk(Chunk* chunk);
void write_chunk(Chunk* chunk, uint8_t byte, int line);
// Reallocate the chunk to null pointer, and then init it.
void free_chunk(Chunk* chunk);
// Add a constant into constant pool in Chunk, then return the index of the added constant.
// The OP_CONSTANT takes 2 bytes, in which the first byte is the enum and the second is
// the index of the constant value it carries in constant pool.
int add_constant(Chunk* chunk, Value val);

#endif