#ifndef LOX_SCANNER_H_
#define LOX_SCANNER_H_

typedef struct{
  // the position of char that current processing token starts with.   
  // NOT the beginning of source
  char const* start;
  // the position of char now scanner points to
  char const* current;
  int line;
}Scanner;

typedef enum{
  // Single-character tokens.
  TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
  TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
  TOKEN_COMMA, TOKEN_DOT, TOKEN_MINUS, TOKEN_PLUS,
  TOKEN_SEMICOLON, TOKEN_SLASH, TOKEN_STAR,
  // One or two character tokens.
  TOKEN_BANG, TOKEN_BANG_EQUAL,
  TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
  TOKEN_GREATER, TOKEN_GREATER_EQUAL,
  TOKEN_LESS, TOKEN_LESS_EQUAL,
  // Literals.
  TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_NUMBER,
  // Keywords.
  TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
  TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
  TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
  TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,

  TOKEN_ERROR, TOKEN_EOF
}TokenType;

// The Token only stores pointer to the corresponding chars in source.  
// It won't copy those chars from original source.
typedef struct{
  // type of token
  TokenType type;
  // from which position in source it starts
  const char* start;
  // its length, by chars
  int length;
  // in which line it appears in source
  int line;
}Token;

Scanner* init_scanner(const char* source);
// scan one token from source
Token scan_token(Scanner* scanner);

#endif