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

  constant = add_constant(&chunk, 3.4);
  write_chunk(&chunk, OP_CONSTANT, 123);
  write_chunk(&chunk, constant, 123);

  write_chunk(&chunk, OP_ADD, 123);

  constant = add_constant(&chunk, 5.6);
  write_chunk(&chunk, OP_CONSTANT, 123);
  write_chunk(&chunk, constant, 123);

  write_chunk(&chunk, OP_DIV, 123);

  write_chunk(&chunk, OP_NEG, 4);
  write_chunk(&chunk, OP_RETURN, 4);
  // disassemble_chunk(&chunk, "test chunk");

  interpret(vm, &chunk);

  free_VM(vm);
  free_chunk(&chunk);

  return 0;
}