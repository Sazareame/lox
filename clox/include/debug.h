#ifndef LOX_DEBUG_H_
#define LOX_DEBUG_H_

#include"chunk.h"

#define DEBUG_TRACE_EXECUTION
#define DEBUG_PRINT_CODE

// Disassemble all of the instruction in chunk and print it to stdout, lead by given name.
void disassemble_chunk(Chunk* chunk, char const* name);
// Disassemble an instruction which stored in chunk and has offset.
int disassemble_instruction(Chunk* chunk, int offset);

#endif