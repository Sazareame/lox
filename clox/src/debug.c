#include <stdio.h>
#include "debug.h"
#include "chunk.h"
#include "value.h"

static int simple_instruction(char const*, int);
static int constant_instruction(char const*, Chunk*, int);

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
  // If the line number is the same with the previous instruction,
  // just omit the line number but ensure they are well-aligned.
  if(offset && chunk->lines[offset] == chunk->lines[offset - 1])
    printf("     ");
  else
    printf("%4d ", chunk->lines[offset]);
  switch(ins){
    case OP_RETURN: return simple_instruction("OP_RETURN", offset);
    case OP_CONSTANT: return constant_instruction("OP_CONSTANT", chunk, offset);
    case OP_NIL: return simple_instruction("OP_NEG", offset);
    case OP_TRUE: return simple_instruction("OP_TRUE", offset);
    case OP_FALSE: return simple_instruction("OP_FALSE", offset);
    case OP_NEG: return simple_instruction("OP_NEG", offset);
    case OP_ADD: return simple_instruction("OP_ADD", offset);
    case OP_SUB: return simple_instruction("OP_SUB", offset);
    case OP_MUL: return simple_instruction("OP_MUL", offset);
    case OP_DIV: return simple_instruction("OP_DIV", offset);
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

static int
constant_instruction(char const* name, Chunk* chunk, int offset){
  uint8_t constant = chunk->code[offset + 1];
  // Print the OpCode and the index of the constant it carries in constant pool.
  printf("%-16s %4d '", name, constant);
  // Print the constant literal itself.
  print_value(chunk->constants.values[constant]);
  printf("\n");
  return offset + 2;
}