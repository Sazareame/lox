#ifndef LOX_DEBUG_H_
#define LOX_DEBUG_H_

#include"chunk.h"

void disassemble_chunk(Chunk* chunk, char const* name);
int disassemble_instruction(Chunk* chunk, int offset);

#endif