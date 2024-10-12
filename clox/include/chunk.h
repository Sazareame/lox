#ifndef CHUNK_H_
#define CHUNK_H_

#include "common.h"
#include "value.h"

typedef enum{
  OP_RETURN,
  OP_CONSTANT,
} OpCode;

typedef struct{
  int count;
  int capacity;
  uint8_t* code;
  ValueArray constants;
}Chunk;

void init_chunk(Chunk* chunk);
void write_chunk(Chunk* chunk, uint8_t byte);
// Reallocate the chunk to null pointer, and then init it.
void free_chunk(Chunk* chunk);
// Add a constant into constant pool in Chunk, then return the index of the added constant.
// The OP_CONSTANT takes 2 bytes, in which the first byte is the enum and the second is
// the index of the constant value it carries in constant pool.
int add_constant(Chunk* chunk, Value val);

#endif