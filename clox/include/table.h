#ifndef CLOX_TABLE_H_
#define CLOX_TABLE_H_

#include "value.h"

typedef struct{
  ObjString* key;
  Value value;
}Entry;

typedef struct {
  int count;
  int capacity;
  Entry* entries;
}Table;

// Init a hash table
void init_table(Table* table);
// Free a hash table
void free_table(Table* table);
// insert a key-value pair to table, update old value if the key already existed and return false, otherwise
// create a new entry and return true
bool table_set(Table* table, ObjString* key, Value value);

#endif
