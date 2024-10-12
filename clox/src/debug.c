#include <stdio.h>
#include "debug.h"
#include "chunk.h"

static int simple_instruction(char const*, int);

void
disassemble_chunk(Chunk *chunk, const char *name){
  fprintf(stdout, "== %s ==\n", name);
  for(int offset = 0; offset < chunk->count;){
    offset = disassemble_instruction(chunk, offset);
  }
}

int
disassemble_instruction(Chunk *chunk, int offset){
  printf("%04d ", offset);
  uint8_t ins = chunk->code[offset];
  switch(ins){
    case OP_RETURN: return simple_instruction("OP_RETURN", offset);
    default:
      printf("unknown opcode %d\n", ins);
      return offset + 1;
  }
}

static int
simple_instruction(char const* name, int offset){
  printf("%s\n", name);
  return offset + 1;
}