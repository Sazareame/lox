#include "scanner.h"
#include <stdlib.h>
#include <assert.h>
#include <stdbool.h>
#include <string.h>

static bool is_at_end(Scanner*);
static Token make_token(Scanner*, TokenType);
static Token error_token(Scanner*, char const*);
static char advance(Scanner*);
static bool match(Scanner*, char);
static char peek(Scanner*);
static char peek_next(Scanner*);
static void skip_whitespace(Scanner*);
static Token string_token(Scanner*);
static bool is_digit(char);
static Token number(Scanner*);
static bool is_legal_ident(char);
static Token ident(Scanner*);
static TokenType ident_type(Scanner*);
static TokenType check_keyword(Scanner*, int, int, char const*, TokenType);

Scanner*
init_scanner(char const* source){
  Scanner* scanner = (Scanner*)malloc(sizeof(Scanner));
  assert(scanner);

  scanner->current = source;
  scanner->start = source;
  scanner->line = 1;

  return scanner;
}

Token
scan_token(Scanner *scanner){
  // TODO unexpect character after reaching the eof.
  skip_whitespace(scanner);
  scanner->start = scanner->current;
  if(is_at_end(scanner)) return make_token(scanner, TOKEN_EOF);
  char c = advance(scanner);

  if(is_digit(c)) return number(scanner);
  if(is_legal_ident(c)) return ident(scanner);

  switch(c){
    case '(': return make_token(scanner, TOKEN_LEFT_PAREN);
    case ')': return make_token(scanner, TOKEN_RIGHT_PAREN);
    case '{': return make_token(scanner, TOKEN_LEFT_BRACE);
    case '}': return make_token(scanner, TOKEN_RIGHT_BRACE);
    case ';': return make_token(scanner, TOKEN_SEMICOLON);
    case ',': return make_token(scanner, TOKEN_COMMA);
    case '.': return make_token(scanner, TOKEN_DOT);
    case '-': return make_token(scanner, TOKEN_MINUS);
    case '+': return make_token(scanner, TOKEN_PLUS);
    case '/': return make_token(scanner, TOKEN_SLASH);
    case '*': return make_token(scanner, TOKEN_STAR);
    case '!': return make_token(scanner, match(scanner, '=') ? TOKEN_BANG_EQUAL : TOKEN_BANG);
    case '=': return make_token(scanner, match(scanner, '=') ? TOKEN_EQUAL_EQUAL : TOKEN_EQUAL);
    case '<': return make_token(scanner, match(scanner, '=') ? TOKEN_LESS_EQUAL : TOKEN_LESS);
    case '>': return make_token(scanner, match(scanner, '=') ? TOKEN_GREATER_EQUAL : TOKEN_GREATER);
    case '"': return string_token(scanner);
  }

  return error_token(scanner, "unexpect character");
}

static bool
is_at_end(Scanner* scanner){
  return *scanner->current == '\0';
}

static Token
make_token(Scanner* scanner, TokenType type){
  Token token;
  token.type = type;
  token.start = scanner->start;
  token.length = (int)(scanner->current - scanner->start);
  token.line = scanner->line;
  return token;
}

static Token
error_token(Scanner* scanner, char const* msg){
  Token token;
  token.type = TOKEN_ERROR;
  token.start = msg;
  token.length = (int)strlen(msg);
  token.line = scanner->line;
  return token;
}

static char
advance(Scanner* scanner){
  return *scanner->current++;
}

static bool
match(Scanner* scanner, char expected){
  if(is_at_end(scanner)) return false;
  if(*scanner->current != expected) return false;
  ++scanner->current;
  return true;
}

static char
peek(Scanner* scnner){
  return *scnner->current;
}

static char
peek_next(Scanner* scanner){
  return is_at_end(scanner) ? '\0' : scanner->current[1];
}

static void 
skip_whitespace(Scanner* scanner){
  char c;
  while(1){
    c = peek(scanner);
    switch(c){
      case ' ':
      case '\r':
      case '\t': advance(scanner); break;
      case '\n': ++scanner->line; advance(scanner); break;
      case '/':
        if(peek_next(scanner) == '/')
          while(peek(scanner) != '\n' && !is_at_end(scanner))
            advance(scanner);
        else return;
        break;
      default: return;
    }
  }
}

static Token
string_token(Scanner* scanner){
  while(peek(scanner) != '"' && !is_at_end(scanner)){
    if(peek(scanner) == '\n') ++scanner->line;
    advance(scanner);
  }

  if(is_at_end(scanner)) return error_token(scanner, "unterminated string");
  advance(scanner);
  return make_token(scanner, TOKEN_STRING);
}

static bool
is_digit(char c){
  return c >= '0' && c <= '9';
}

static Token
number(Scanner* scanner){
  while(is_digit(peek(scanner))) advance(scanner);
  if(peek(scanner) == '.' && is_digit(peek_next(scanner)))
    do{advance(scanner);}while(is_digit(peek(scanner)));
  return make_token(scanner, TOKEN_NUMBER);
}

static bool
is_legal_ident(char c){
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

static Token
ident(Scanner* scanner){
  while(is_legal_ident(peek(scanner)) || is_digit(peek(scanner))) advance(scanner);
  return make_token(scanner, ident_type(scanner));
}

static TokenType 
ident_type(Scanner* scanner){
  switch (scanner->start[0]) {
    case 'a': return check_keyword(scanner, 1, 2, "nd", TOKEN_AND);
    case 'c': return check_keyword(scanner, 1, 4, "lass", TOKEN_CLASS);
    case 'e': return check_keyword(scanner, 1, 3, "lse", TOKEN_ELSE);
    case 'i': return check_keyword(scanner, 1, 1, "f", TOKEN_IF);
    case 'n': return check_keyword(scanner, 1, 2, "il", TOKEN_NIL);
    case 'o': return check_keyword(scanner, 1, 1, "r", TOKEN_OR);
    case 'p': return check_keyword(scanner, 1, 4, "rint", TOKEN_PRINT);
    case 'r': return check_keyword(scanner, 1, 5, "eturn", TOKEN_RETURN);
    case 's': return check_keyword(scanner, 1, 4, "uper", TOKEN_SUPER);
    case 'v': return check_keyword(scanner, 1, 2, "ar", TOKEN_VAR);
    case 'w': return check_keyword(scanner, 1, 4, "hile", TOKEN_WHILE);
    case 'f':
      if(scanner->current - scanner->start > 1) {
        switch(scanner->start[1]){
          case 'a': return check_keyword(scanner, 2, 3, "lse", TOKEN_FALSE);
          case 'o': return check_keyword(scanner, 2, 1, "r", TOKEN_FOR);
          case 'u': return check_keyword(scanner, 2, 1, "n", TOKEN_FUN);
        }
      }
      break;
    case 't':
      if(scanner->current - scanner->start > 1){
        switch(scanner->start[1]){
          case 'h': return check_keyword(scanner, 2, 2, "is", TOKEN_THIS);
          case 'r': return check_keyword(scanner, 2, 2, "ue", TOKEN_TRUE);
        }
      }
      break;
  }
  return TOKEN_IDENTIFIER;
}

static TokenType
check_keyword(Scanner* scanner, int start, int len, char const* rest, TokenType type){
  if(scanner->current - scanner->start == start + len && !memcmp(scanner->start + start, rest, len))
    return type;
  return TOKEN_IDENTIFIER;
}