#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "vm.h"

int
main(int argc, char const** argv){
  VM* vm;
  init_VM(&vm);

  Chunk chunk;
  init_chunk(&chunk);

  int constant =  add_constant(&chunk, 1.2);
  write_chunk(&chunk, OP_CONSTANT, 2);
  write_chunk(&chunk, constant, 2);
  write_chunk(&chunk, OP_RETURN, 3);
  disassemble_chunk(&chunk, "test chunk");

  interpret(vm, &chunk);

  free_VM(vm);
  free_chunk(&chunk);

  return 0;
}