#include "chunk.h"
#include "common.h"
#include "debug.h"
#include "vm.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char* read_file(char const*);
static void run_file(VM*, char const*);
static void repl(VM*);

int
main(int argc, char const** argv){
  VM* vm;
  init_VM(&vm);

  if(argc == 1){
    repl(vm);
  }else if(argc == 2){
    run_file(vm, argv[1]);
  }else{
    fprintf(stderr, "Usage: lox <path>\n");
    exit(64);
  }

  free_VM(vm);
  return 0;
}

static char*
read_file(char const* path){
  FILE* file;
  if(!(file = fopen(path, "rb"))){
    fprintf(stderr, "Error when loading file: %s", path);
    exit(74);
  }

  fseek(file, 0L, SEEK_END);
  size_t file_size = ftell(file);
  rewind(file);

  char* buf = (char*)malloc(file_size + 1);
  if(!buf){
    fprintf(stderr, "not enough memory to allocate for bufsize %lu", file_size);
    exit(74);
  }

  size_t bytes = fread(buf, sizeof(char), file_size, file);
  if(bytes < file_size){
    fprintf(stderr, "Error: expect to read %lu, but get %lu", file_size, bytes);
    exit(74);
  }

  buf[bytes] = '\0';
  return buf;
}

static void
run_file(VM* vm, char const* path){
  char* source = read_file(path);
  InterpretResult res = interpret(vm, source);
  free(source);

  if(res == INTERPRET_COMPILE_ERROR) exit(65);
  if(res == INTERPRET_RUNTIME_ERROR) exit(70);
}

static void
repl(VM* vm){
  char line[1024];
  while(1){
    printf("> ");

    if(!fgets(line, sizeof(line), stdin)){
      printf("\n");
      break;
    }

    interpret(vm, line);
  }
}