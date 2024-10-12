#ifndef CHUNK_H_
#define CHUNK_H_

#include "common.h"

typedef enum{
  OP_RETURN,
} OpCode;

typedef struct{
  uint8_t* code;
}Chunk;

#endif