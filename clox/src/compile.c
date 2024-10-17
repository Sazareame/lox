#include "compile.h"
#include "scanner.h"
#include <stdio.h>
#include <stdlib.h>
#include "debug.h"
#include "object.h"

typedef struct{
  Token current;
  Token previous;
  bool had_error;
  bool panic_mode;
}Parser;

typedef enum {
  PREC_NONE,
  PREC_ASSIGNMENT,  // =
  PREC_OR,          // or
  PREC_AND,         // and
  PREC_EQUALITY,    // == !=
  PREC_COMPARISON,  // < > <= >=
  PREC_TERM,        // + -
  PREC_FACTOR,      // * /
  PREC_UNARY,       // ! -
  PREC_CALL,        // . ()
  PREC_PRIMARY
} Precedence;

typedef void (*ParseFn)(Scanner*);

typedef struct{
  ParseFn prefix;
  ParseFn infix;
  Precedence precedence;
}ParseRule;


Parser parser;

Chunk* compiling_chunk;

// to parse next token
static void advance(Scanner*);
// same effect as advance() with type checking, which requires the next token has
// the given type.
static void consume(Scanner*, TokenType, char const*);
// to report error at current token
static void error_at_current(char const*);
// to report error at previous token
static void error(char const*);
// to report error at the given token
static void error_at(Token*, char const*);

// emit bytecode to chunk, with the line of previous token
static void emit_byte(uint8_t byte);
// emit two bytecode
static void emit_bytes(uint8_t, uint8_t);
// return the current working chunk
static Chunk* current_chunk();
// endup work aftering most of the procedure of compiling
static void end_compile();
// emit a OP_RETURN
static void emit_return();

static void number();
static void emit_const(Value);
static uint8_t make_const(Value);

static ParseRule* get_rule(TokenType type);
// parse expresions which have the given or higher precedence.
static void parse_precedence(Scanner*, Precedence);
// parse an expression
static void expression(Scanner*);
// parse an expression wrapped by a couple if parentheses
static void grouping(Scanner*);
static void unary(Scanner*);
static void binary(Scanner*);
static void literal(Scanner*);
static void string(Scanner*);

/*
this table has three columns, which are:
the function to call to parse the prefix expression lead by given token,
the function to call to parse the midfix expression with the given token as operator,
the precedence of the corresponding operator of the given token, if exists.
respectively.
the null function pointer means that there should not be the very situation, thus
if the visitor get a null pointer, it should be regarded as a parse error.
*/
ParseRule rules[] = {
  [TOKEN_LEFT_PAREN]    = {grouping, NULL,   PREC_NONE},
  [TOKEN_RIGHT_PAREN]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_LEFT_BRACE]    = {NULL,     NULL,   PREC_NONE}, 
  [TOKEN_RIGHT_BRACE]   = {NULL,     NULL,   PREC_NONE},
  [TOKEN_COMMA]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_DOT]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_MINUS]         = {unary,    binary, PREC_TERM},
  [TOKEN_PLUS]          = {NULL,     binary, PREC_TERM},
  [TOKEN_SEMICOLON]     = {NULL,     NULL,   PREC_NONE},
  [TOKEN_SLASH]         = {NULL,     binary, PREC_FACTOR},
  [TOKEN_STAR]          = {NULL,     binary, PREC_FACTOR},
  [TOKEN_BANG]          = {unary,     NULL,   PREC_NONE},
  [TOKEN_BANG_EQUAL]    = {NULL,     binary,   PREC_EQUALITY},
  [TOKEN_EQUAL]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_EQUAL_EQUAL]   = {NULL,     binary,   PREC_EQUALITY},
  [TOKEN_GREATER]       = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_GREATER_EQUAL] = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_LESS]          = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_LESS_EQUAL]    = {NULL,     binary,   PREC_COMPARISON},
  [TOKEN_IDENTIFIER]    = {NULL,     NULL,   PREC_NONE},
  [TOKEN_STRING]        = {string,     NULL,   PREC_NONE},
  [TOKEN_NUMBER]        = {number,   NULL,   PREC_NONE},
  [TOKEN_AND]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_CLASS]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_ELSE]          = {NULL,     NULL,   PREC_NONE},
  [TOKEN_FALSE]         = {literal,     NULL,   PREC_NONE},
  [TOKEN_FOR]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_FUN]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_IF]            = {NULL,     NULL,   PREC_NONE},
  [TOKEN_NIL]           = {literal,     NULL,   PREC_NONE},
  [TOKEN_OR]            = {NULL,     NULL,   PREC_NONE},
  [TOKEN_PRINT]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_RETURN]        = {NULL,     NULL,   PREC_NONE},
  [TOKEN_SUPER]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_THIS]          = {NULL,     NULL,   PREC_NONE},
  [TOKEN_TRUE]          = {literal,     NULL,   PREC_NONE},
  [TOKEN_VAR]           = {NULL,     NULL,   PREC_NONE},
  [TOKEN_WHILE]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_ERROR]         = {NULL,     NULL,   PREC_NONE},
  [TOKEN_EOF]           = {NULL,     NULL,   PREC_NONE},
};

bool
compile(const char *source, Chunk* chunk){
  Scanner* scanner = init_scanner(source);
  compiling_chunk = chunk;

  parser.had_error = false;
  parser.panic_mode = false;

  advance(scanner);
  expression(scanner);
  consume(scanner, TOKEN_EOF, "expect end of expression");
  end_compile();
  return !parser.had_error;
}

static void
advance(Scanner* scanner){
  parser.previous = parser.current;
  while(1){
    parser.current = scan_token(scanner);
    if(parser.current.type != TOKEN_ERROR) break;
    error_at_current(parser.current.start);
  }
}

static void
error_at_current(char const* msg){
  error_at(&parser.current, msg);
}

static void
error(char const* msg){
  error_at(&parser.previous, msg);
}

static void
error_at(Token* token, char const* msg){
  if(parser.panic_mode) return;
  parser.panic_mode = true;
  fprintf(stderr, "[line %d] Error", token->line);

  if(token->type == TOKEN_EOF){
    fprintf(stderr, " at end");
  }else if(token->type == TOKEN_ERROR){
  }else{
    fprintf(stderr, " at '%.*s'", token->length, token->start);
  }

  fprintf(stderr, ": %s\n", msg);

  parser.had_error = true;
}

static void
consume(Scanner* scanner, TokenType type, char const* msg){
  if(parser.current.type == type){
    advance(scanner);
    return;
  }
  error_at_current(msg);
}

static void
emit_byte(uint8_t byte){
  write_chunk(current_chunk(), byte, parser.previous.line);
}

static Chunk*
current_chunk(){
  return compiling_chunk;
}

static void
end_compile(){
  emit_return();
#ifdef DEBUG_PRINT_CODE
  if(!parser.had_error)
    disassemble_chunk(current_chunk(), "code");
#endif
}

static void
emit_return(){
  emit_byte(OP_RETURN);
}

static void
emit_bytes(uint8_t byte1, uint8_t byte2){
  emit_byte(byte1);
  emit_byte(byte2);
}

static void
number(){
  double value = strtod(parser.previous.start, NULL);
  emit_const(NUMBER_VAL(value));
}

static void
emit_const(Value value){
  emit_bytes(OP_CONSTANT, make_const(value));
}

static uint8_t
make_const(Value value){
  int constant = add_constant(current_chunk(), value);
  if(constant > UINT8_MAX){
    error("too many constants in one chunk");
    return 0;
  }
  return (uint8_t)constant;
}

static void
expression(Scanner* scanner){
  parse_precedence(scanner, PREC_ASSIGNMENT);
}

static void
grouping(Scanner* scanner){
  expression(scanner);
  consume(scanner, TOKEN_RIGHT_PAREN, "expect ')' after expression");
}

static void
unary(Scanner* scanner){
  TokenType op_type = parser.previous.type;
  parse_precedence(scanner, PREC_UNARY);
  switch(op_type){
    case TOKEN_MINUS: emit_byte(OP_NEG); break;
    case TOKEN_BANG: emit_byte(OP_NOT); break;
    default: return;
  }
}

static void
binary(Scanner* scanner){
  TokenType op_type = parser.previous.type;
  ParseRule* rule = get_rule(op_type);
  // parse the expresion whose precedence is higher than current op
  // because the binary opration is left associated.
  parse_precedence(scanner, (Precedence)(rule->precedence + 1));

  switch(op_type){
    case TOKEN_PLUS: emit_byte(OP_ADD); break;
    case TOKEN_MINUS: emit_byte(OP_SUB); break;
    case TOKEN_STAR: emit_byte(OP_MUL); break;
    case TOKEN_SLASH: emit_byte(OP_DIV); break;
    case TOKEN_BANG_EQUAL: emit_bytes(OP_EQUAL, OP_NOT); break;
    case TOKEN_EQUAL_EQUAL: emit_byte(OP_EQUAL); break;
    case TOKEN_GREATER: emit_byte(OP_GREATER); break;
    case TOKEN_GREATER_EQUAL: emit_bytes(OP_LESS, OP_NOT); break;
    case TOKEN_LESS: emit_byte(OP_LESS); break;
    case TOKEN_LESS_EQUAL: emit_bytes(OP_GREATER, OP_NOT); break;
    default: return;
  }
}

static void
literal(Scanner* scanner){
  switch(parser.previous.type){
    case TOKEN_FALSE: emit_byte(OP_FALSE); break;
    case TOKEN_NIL: emit_byte(OP_NIL); break;
    case TOKEN_TRUE: emit_byte(OP_TRUE); break;
    default: return;
  }
}

static void
string(Scanner* scanner){
  emit_const(OBJ_VAL(copy_string(parser.previous.start + 1, parser.previous.length - 2)));
}

static ParseRule*
get_rule(TokenType type){
  return &rules[type];
}

static void
parse_precedence(Scanner* scanner, Precedence precedence){
  advance(scanner);
  ParseFn prefix_rule = get_rule(parser.previous.type)->prefix;
  // there must be an unary expression or just a number, which both obtain a
  // prefix function to parse it.
  // Therefore, the situation prefix function pointer is null, means that 
  // something such as ` + 1` etc., which should be an error due to there is 
  // no expression befor `+` operator.
  if(!prefix_rule){
    error("expect expression");
    return;
  }
  prefix_rule(scanner);

  /*
    after precessing the prefix expression, such as `-1` in `-1 + expression`,
    the `current` token now comes to `+` token for infix parsing.
    if the precedence of `current` token, i.e. `+`, is lower that given argument `precedence`, 
    then it should not be parsed now, so the function returns. 
    e.g. 1 * 2 + 3, in such case, the argument precedence is that of `*`, and current token `+`
    has lower precedence, so the function returns.
    otherwise, e.g. 1 + 2 * 3, the argument precedence is that of `+`, which is lower than that of `*`,
    so that this function successively jumps into infix rule to parse the `*` binary expression.
    after parsing the `*` binary expression, the while statement continue judging whether the next
    token has higher precedence (now the scanner has come to the token after * and its oprand),
    in `1 + 2 * 3`, the next token is EOF, which has the lowest precedence, so the function returns.
    If the expression is `1 + 2 * 3 / 4`, the code will still be in the while loop and precess the `/` token.
  */
  ParseFn infix_rule = 0;
  // infix expression
  while(precedence <= get_rule(parser.current.type)->precedence){
    advance(scanner);
    infix_rule = get_rule(parser.previous.type)->infix;
    infix_rule(scanner);
  }
}