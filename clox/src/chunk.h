/** Module defining code representation. Chunks refer to sequences of bytecode.
 */
#ifndef clox_chunk_h
#define clox_chunk_h

#include "common.h"
#include "value.h"

/** Instruction Set */
typedef enum {
  OP_CONSTANT,
  OP_RETURN,
} OpCode;

/** Sequence of instructions */
typedef struct {
  int count;
  int capacity;
  uint8_t* code;
  int* lines;
  ValueArray constants;
} Chunk;

void initChunk(Chunk* chunk);
void freeChunk(Chunk* chunk);
void writeChunk(Chunk* chunk, uint8_t byte, int line);
int addConstant(Chunk* chunk, Value value);

#endif