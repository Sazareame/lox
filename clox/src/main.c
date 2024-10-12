#include "chunk.h"
#include "common.h"
#include "debug.h"

int
main(int argc, char const** argv){
  Chunk chunk;
  init_chunk(&chunk);
  write_chunk(&chunk, OP_RETURN, 1);

  int constant =  add_constant(&chunk, 1.2);
  write_chunk(&chunk, OP_CONSTANT, 2);
  write_chunk(&chunk, constant, 2);
  write_chunk(&chunk, OP_RETURN, 2);
  write_chunk(&chunk, OP_RETURN, 3);
  disassemble_chunk(&chunk, "test chunk");
  free_chunk(&chunk);
  return 0;
}