#ifndef LOX_COMPILE_H_
#define LOX_COMPILE_H_

#include "vm.h"

// handle the parse and compile process.
// return false if there was any error during processing.
bool compile(char const* source, Chunk* chunk);

#endif