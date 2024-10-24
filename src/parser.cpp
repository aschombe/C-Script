#include "../include/parser.hpp"
#include "../include/ast.hpp"
#include "../include/types.hpp"
#include "../include/error_handler.hpp"
#include <iostream>
#include <regex>

bool Parser::is_keyword(const Token& token) {
  return token.type == LET || token.type == DEL || token.type == IF || token.type == FOR || token.type == WHILE || token.type == BREAK || token.type == CONTINUE || token.type == RETURN || token.type == EXIT || token.type == FUNC || token.type == SWITCH || token.type == IMPORT || token.type == STRUCT_DEF;
}

bool Parser::is_assignment(const Token& token) {
  return token.type == ASSIGN || token.type == ADD_ASSIGN || token.type == SUB_ASSIGN || token.type == MUL_ASSIGN || token.type == DIV_ASSIGN || token.type == MOD_ASSIGN || token.type == POW_ASSIGN;
}

std::vector<ASTNode*> Parser::parse() {
  while (current < tokens.size()) {
    ASTNode* node;
    if (is_keyword(tokens[current]) || is_assignment(tokens[current])) {
      node = parse_keyword();
    } else {
      node = parse_expression();
    }

    if (node) {
      ast.push_back(node);
    } else {
      std::cerr << "Error: Failed to create AST node: " << std::endl;
      token_to_string(tokens[current]);
    }
  }

  return ast;
}

ASTNode* Parser::parse_keyword() {
  switch (tokens[current].type) {
    case IMPORT: return parse_import();
    case LET: return parse_let();
    case ASSIGN: return parse_assignment();
    case ADD_ASSIGN: return parse_assignment();
    case SUB_ASSIGN: return parse_assignment();
    case MUL_ASSIGN: return parse_assignment();
    case DIV_ASSIGN: return parse_assignment();
    case MOD_ASSIGN: return parse_assignment();
    case POW_ASSIGN: return parse_assignment();
    case DEL: return parse_del();
    case IF: return parse_if();
    case FOR: return parse_for();
    case WHILE: return parse_while();
    case BREAK: return parse_break();
    case CONTINUE: return parse_continue();
    case RETURN: return parse_return();
    case EXIT: return parse_exit();
    case FUNC: return parse_func();
    case SWITCH: return parse_switch();
    /* case STRUCT_DEF: return parse_struct_def(); */
    default: return parse_expression();
  }
}

ASTNode* Parser::parse_import() {
  // import -> "<filepath>";
  current++; // consume "import"
  std::string rel_fpath = tokens[current].value;
  current++; // filepath
  current++; // consume ;
  return new ImportNode(rel_fpath, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_struct_def() {
  // struct <name> {
  //  <field_name>: type;
  //  <field_name>: type;
  // };
  current++; // consume struct
  std::string name = tokens[current].value;
  current++; // consume name
  current++; // consume ;
  std::unordered_map<std::string, Var_Types> fields;
  std::string f_name;
  std::string f_type;
  while (tokens[current].value != "}") {
    f_name = tokens[current].value;
    current++; // consume field name
    current++; // consume :
    f_type = tokens[current].value;
    current++; // consume type
    fields[f_name] = string_to_var_type(f_type);
    if (tokens[current].value != ";") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after field in struct definition", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume ;
  }
  current++; // consume }
  current++; // consume ;
  return new StructDef(name, fields, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_let() {
  // let -> "let" IDENTIFIER: TYPE "=" expression ";"
  current++; // consume "let"
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  current++; // consume ":"
  std::string type = tokens[current].value;
  Var_Types var_type = string_to_var_type(type);
  current++; // consume TYPE
  if (tokens[current].value != "=") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '=' after type in let statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "="
  ASTNode* expression = parse_expression();
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after expression in let statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  return new LetNode(identifier, var_type, expression, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_del() {
  // del -> "del" IDENTIFIER ";"
  current++; // consume "del"
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after identifier in del statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  return new DelNode(identifier, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_if() {
  // if -> "if" (expression) "{" ( keyword | expression )* "}" ( "elif" (expression) "{" ( keyword | expression )* "}" )* ( "else" "{" ( keyword | expression )* "}" )?
  current++; // consume "if"
  if (tokens[current].value != "(") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after 'if'", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "("
  ASTNode* if_condition = parse_expression();
  if (tokens[current].value != ")") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after if condition", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after if condition", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "{"
  std::vector<ASTNode*> if_body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after if body", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    if (is_keyword(tokens[current])) {
      if_body.push_back(parse_keyword());
    } else {
      if_body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  // parse elifs, if any
  std::vector<std::pair<ASTNode*, std::vector<ASTNode*>>> elifs;
  while (current < tokens.size() && tokens[current].value == "elif") {
    current++; // consume "elif"
    if (tokens[current].value != "(") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after 'elif'", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume "("
    ASTNode* elif_condition = parse_expression();
    if (tokens[current].value != ")") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after elif condition", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume ")"
    if (tokens[current].value != "{") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after elif condition", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume "{"
    std::vector<ASTNode*> elif_body;
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after elif body", tokens[current].line, tokens[current].col, tokens[current].snippet};
        throw error;
      }
      if (is_keyword(tokens[current])) {
        elif_body.push_back(parse_keyword());
      } else {
        elif_body.push_back(parse_expression());
      }
    }
    current++; // consume "}"
    elifs.push_back(std::make_pair(elif_condition, elif_body));
  }
  // parse else, if any
  std::vector<ASTNode*> else_body;
  if (current < tokens.size() && tokens[current].value == "else") {
    current++; // consume "else"
    if (tokens[current].value != "{") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after else", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume "{"
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after else body", tokens[current].line, tokens[current].col, tokens[current].snippet};
        throw error;
      }
      if (is_keyword(tokens[current])) {
        else_body.push_back(parse_keyword());
      } else {
        else_body.push_back(parse_expression());
      }
    }
    current++; // consume "}"
  }
  return new IEENode(if_condition, if_body, elifs, else_body, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_for() {
  // for -> "for" (IDENTIFIER; expression; expression) "{" ( keyword | expression )* "}"
  current++; // consume "for"
  if (tokens[current].value != "(") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after 'for'", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "("
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after identifier in for statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  ASTNode* condition = parse_expression();
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after condition in for statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  ASTNode* increment = parse_expression();
  if (tokens[current].value != ")") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after increment in for statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after for statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "{"
  std::vector<ASTNode*> body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after for body", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    if (is_keyword(tokens[current])) {
      body.push_back(parse_keyword());
    } else {
      body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  return new ForNode(identifier, condition, increment, body, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_while() {
  // while -> "while" (expression) "{" ( keyword | expression )* "}"
  current++; // consume "while"
  if (tokens[current].value != "(") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after 'while'", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "("
  ASTNode* condition = parse_expression();
  if (tokens[current].value != ")") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after while condition", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after while condition", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "{"
  std::vector<ASTNode*> body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after while body", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    if (is_keyword(tokens[current])) {
      body.push_back(parse_keyword());
    } else {
      body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  return new WhileNode(condition, body, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_break() {
  // break -> "break" ";"
  current++; // consume "break"
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after 'break'", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  return new BreakNode(tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_continue() {
  // continue -> "continue" ";"
  current++; // consume "continue"
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after 'continue'", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  return new ContinueNode(tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_return() {
  // return -> "return" expression ";"
  std::cout << "Current token: " << tokens[current].value << std::endl;
  current++; // consume "return"
  std::cout << "Current token: " << tokens[current].value << std::endl;
  ASTNode* expression = parse_expression();

  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after expression in return statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  std::cout << "Current token: " << tokens[current].value << std::endl;
  current++; // consume ";"
  std::cout << "Current token: " << tokens[current].value << std::endl;
  return new ReturnNode(expression, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_exit() {
  // exit -> "exit" <expression> ";"
  current++; // consume "exit"
  ASTNode* expression = parse_expression();
  if (tokens[current].value != ";") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ';' after expression in exit statement", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ";"
  return new ExitNode(expression, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_func() {
  // func -> "func" IDENTIFIER "(" (IDENTIFIER: TYPE ("," IDENTIFIER: TYPE)*)? "): " <return_type> "{" ( keyword | expression )* "}"
  // func IDENT(arg1: TYPE, arg2: TYPE): TYPE { ... }
  current++; // consume "func"
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  if (tokens[current].value != "(") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after function identifier", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "("
  std::vector<std::pair<std::string, Var_Types>> args;
  if (tokens[current].value != ")") {
    do {
      std::string arg_identifier = tokens[current].value;
      current++; // consume IDENTIFIER
      if (tokens[current].value != ":") {
        ErrorHandler error{ErrorType::SYNTACTIC, "Expected ':' after argument identifier in function declaration", tokens[current].line, tokens[current].col, tokens[current].snippet};
        throw error;
      }
      current++; // consume ":"
      std::string arg_type = tokens[current].value;
      current++; // consume TYPE
      args.push_back(std::make_pair(arg_identifier, string_to_var_type(arg_type)));
      if (tokens[current].value == ",") {
        current++; // consume ","
      } else {
        break;
      }
    } while (current < tokens.size());
  }
  if (tokens[current].value != ")") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after function arguments", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ")"
  if (tokens[current].value != ":") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ':' after function arguments closing parenthesis", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ":"
  Func_Types return_type = string_to_func_type(tokens[current].value);
  current++; // consume return type
  if (tokens[current].value != "{") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after function declaration", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "{"
  std::vector<ASTNode*> body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after function body", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    if (is_keyword(tokens[current])) {
      body.push_back(parse_keyword());
    } else {
      body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  return new FuncNode(identifier, return_type, args, body, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_switch() {
  // switch -> "switch" (expression) "{" ( "case" (expression) "{" ( keyword | expression )* "}" )+ ( "default" "{" ( keyword | expression )* "}" )?
  current++; // consume "switch"
  if (tokens[current].value != "(") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after 'switch'", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "("
  ASTNode* expression = parse_expression();
  if (tokens[current].value != ")") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after switch expression", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after switch expression", tokens[current].line, tokens[current].col, tokens[current].snippet};
    throw error;
  }
  current++; // consume "{"
  std::vector<std::pair<ASTNode*, std::vector<ASTNode*>>> cases;
  while (current < tokens.size() && tokens[current].value == "case") {
    current++; // consume "case"
    if (tokens[current].value != "(") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '(' after 'case'", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume "("
    ASTNode* case_expression = parse_expression();
    if (tokens[current].value != ")") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after case expression", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume ")"
    if (tokens[current].value != "{") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after case expression", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume "{"
    std::vector<ASTNode*> case_body;
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after case body", tokens[current].line, tokens[current].col, tokens[current].snippet};
        throw error;
      }
      if (is_keyword(tokens[current])) {
        case_body.push_back(parse_keyword());
      } else {
        case_body.push_back(parse_expression());
      }
    }
    current++; // consume "}"
    cases.push_back(std::make_pair(std::move(case_expression), std::move(case_body)));
  }
  std::vector<ASTNode*> default_body;
  if (current < tokens.size() && tokens[current].value == "default") {
    current++; // consume "default"
    if (tokens[current].value != "{") {
      ErrorHandler error{ErrorType::SYNTACTIC, "Expected '{' after 'default'", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++; // consume "{"
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after default body", tokens[current].line, tokens[current].col, tokens[current].snippet};
        throw error;
      }
      if (is_keyword(tokens[current])) {
        default_body.push_back(parse_keyword());
      } else {
        default_body.push_back(parse_expression());
      }
    }
    current++; // consume "}"
  }
  current++; // consume "}"
  return new SCDNode(expression, cases, default_body, tokens[current].line, tokens[current].col, tokens[current].snippet);
}

ASTNode* Parser::parse_expression() {
  // expression -> assignment ;
  return parse_assignment();
}

ASTNode* Parser::parse_assignment() {
  // assignment -> logical_or ( ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "^=") logical_or )* ;
  ASTNode* node = parse_logical_or();

  while (current < tokens.size()) {
    if (tokens[current].value == "=" || tokens[current].value == "+=" || tokens[current].value == "-=" || tokens[current].value == "*=" || tokens[current].value == "/=" || tokens[current].value == "%=" || tokens[current].value == "^=") {
      std::string ident = tokens[current - 1].value;
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_logical_or();
      current++;

      node = new SetNode(op, ident, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_logical_or() {
  // logical_or -> logical_and ( "||" logical_and )* ;
  ASTNode* node = parse_logical_and();

  while (current < tokens.size()) {
    if (tokens[current].value == "||") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_logical_and();
    
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_logical_and() {
  // logical_and -> equality ( "&&" equality )* ;
  ASTNode* node = parse_equality();

  while (current < tokens.size()) {
    if (tokens[current].value == "&&") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_equality();
      
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_equality() {
  // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
  ASTNode* node = parse_comparison();

  while (current < tokens.size()) {
    if (tokens[current].value == "!=" || tokens[current].value == "==") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_comparison();
      
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_comparison() {
  // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  ASTNode* node = parse_term();

  while (current < tokens.size()) {
    if (tokens[current].value == ">" || tokens[current].value == ">=" || tokens[current].value == "<" || tokens[current].value == "<=") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_term();
      
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_term() {
  // term -> factor ( ( "-" | "+" ) factor )* ;
  ASTNode* node = parse_factor();

  while (current < tokens.size()) {
    if (tokens[current].value == "-" || tokens[current].value == "+") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_factor();
      
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_factor() {
  // factor -> exponentiation ( ( "/" | "*" | "%" ) exponentiation )* ;
  ASTNode* node = parse_exponentiation();

  while (current < tokens.size()) {
    if (tokens[current].value == "/" || tokens[current].value == "*" || tokens[current].value == "%") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_exponentiation();
      
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_exponentiation() {
  // exponentiation -> unary | postfix ( "^" unary | postfix )* ;
  ASTNode* node = parse_unary();

  while (current < tokens.size()) {
    if (tokens[current].value == "^") {
      std::string op = tokens[current].value;
      current++;
      ASTNode* right = parse_unary();
      
      node = new BinOpNode(op, node, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
    } else {
      break;
    }
  }

  return node;
}

ASTNode* Parser::parse_unary() {
  // unary -> ( "!" | "-" ) unary | primary ;
  if (tokens[current].value == "!" || tokens[current].value == "-") {
    std::string op = tokens[current].value;
    current++;
    ASTNode* right = parse_unary();
    
    return new UnaryOpNode(op, right, tokens[current].line, tokens[current].col, tokens[current].snippet);
  } else if (tokens[current].value == "++" || tokens[current].value == "--") {
    std::string ident = tokens[current - 1].value;
    std::string op = tokens[current].value;
    current++;
    // remove previous token
    // ast.erase(ast.begin()+current-1);
    // dont erase the node, just mark the previous AST node to be ignored
    ast[current-1]->ignore = true;

    if (tokens[current].value == ";") {
      current++;
    }
    
    return new PostFixNode(op, ident, tokens[current].line, tokens[current].col, tokens[current].snippet);
  } else {
    return parse_primary();
  }
}

ASTNode* Parser::parse_primary() {
  // primary -> INT | Double | STRING | BOOL | "(" expression ")" | FuncCall | StructInit

  if (tokens[current].value == "(") {
    current++;
    ASTNode* node = parse_expression();
    if (tokens[current].value != ")") {
      // clean up
      delete node;

      ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after expression", tokens[current].line, tokens[current].col, tokens[current].snippet};
      throw error;
    }
    current++;
    return node;
  } else {
    std::string token = tokens[current].value;

    // check for struct initialization
    // <name> { <field>: value, <feld>: value, ..., <field>: value }
    /* if (std::regex_match(token, std::regex("[a-zA-Z_][a-zA-Z0-9_]*")) && current + 1 < tokens.size() && tokens[current + 1].value == "{") { */
    /*   std::string name = token; */
    /*   current++; // consume the identifier */
    /*   current++; // consume the { */
    /*   std::unordered_map<std::string, ASTNode*> fields; */
    /*   std::string f_name; */
    /*   std::string f_type; */
    /*   while (tokens[current].value != "}") { */
    /*     f_name = tokens[current].value; */
    /*     current++; // consume field name */
    /*     if (tokens[current].value != ":") { */
    /*       // clean up */
    /*       for (auto& [key, value] : fields) { */
    /*         delete value; */
    /*       } */

    /*       ErrorHandler error{ErrorType::SYNTACTIC, "Expected ':' after field name in struct initialization", tokens[current].line, tokens[current].col, tokens[current].snippet}; */
    /*       throw error; */
    /*     } */
    /*     current++; // consume : */
    /*     ASTNode* value = parse_expression(); */
    /*     fields[f_name] = value; */
    /*     if (tokens[current].value == ",") { */
    /*       current++; // consume ',' */
    /*     } else { */
    /*       break; */
    /*     } */
    /*   } */
    /*   if (tokens[current].value != "}") { */
    /*     // clean up */
    /*     for (auto& [key, value] : fields) { */
    /*       delete value; */
    /*     } */

    /*     ErrorHandler error{ErrorType::SYNTACTIC, "Expected '}' after struct initialization", tokens[current].line, tokens[current].col, tokens[current].snippet}; */
    /*     throw error; */
    /*   } */
    /*   current++; // consume the '}' */

    /*   return new StructInit(name, fields, tokens[current].line, tokens[current].col, tokens[current].snippet); */
    /* } */

    // Check for struct access
    /* if (std::regex_match(token, std::regex("[a-zA-Z_][a-zA-Z0-9_]*")) && current + 1 < tokens.size() && tokens[current + 1].value == ".") { */
    /*   std::string struct_name = token; */
    /*   current++; // consume the identifier */
    /*   current++; // consume the '.' */
    /*   std::string struct_field = tokens[current].value; */
    /*   current++; // consume the field */
      
    /*   return new StructAccess(struct_name, struct_field, tokens[current].line, tokens[current].col, tokens[current].snippet); */
    /* } */

    // Check for function call
    if (std::regex_match(token, std::regex("[a-zA-Z_][a-zA-Z0-9_]*")) && current + 1 < tokens.size() && tokens[current + 1].value == "(") {
      current++; // consume the identifier
      current++; // consume the '('

      std::vector<ASTNode*> args;
      // Optionally parse arguments if they exist
      if (tokens[current].value != ")") {
        do {
          args.push_back(parse_expression());
          if (tokens[current].value == ",") {
            current++; // consume ','
          } else {
            break;
          }
        } while (current < tokens.size());
      }
      
      if (tokens[current].value != ")") {
        // clean up
        for (auto& arg : args) {
          delete arg;
        }

        ErrorHandler error{ErrorType::SYNTACTIC, "Expected ')' after function arguments", tokens[current].line, tokens[current].col, tokens[current].snippet};
        throw error;
      }
      current++; // consume the ')'

      
      return new CallNode(token, args, tokens[current].line, tokens[current].col, tokens[current].snippet);
    }

    // Check for INT
    if (std::regex_match(token, std::regex("[0-9]+"))) {
      current++;
      
      return new IntNode(std::stoi(token), tokens[current].line, tokens[current].col, tokens[current].snippet);
    }

    // Check for FLOAT
    if (std::regex_match(token, std::regex("[0-9]+\\.[0-9]+"))) {
      current++;
      
      return new DoubleNode(std::stof(token), tokens[current].line, tokens[current].col, tokens[current].snippet);
    }

    // Check for STRING
    if (std::regex_match(token, std::regex("\"[^\"]*\""))) {
      current++;
      
      return new StringNode(token.substr(1, token.size() - 2), tokens[current].line, tokens[current].col, tokens[current].snippet);
    }

    // Check for Bool
    if (token == "true" || token == "false") {
      current++;
      
      return new BoolNode(token == "true", tokens[current].line, tokens[current].col, tokens[current].snippet);
    }

    // If it's not a primary, assume it's a variable
    current++;
    
    return new VariableNode(token, tokens[current].line, tokens[current].col, tokens[current].snippet);
  }
  // reorder the parsing of primary expressions, check for literals first (int, double, string, bool)
  // TODO

  ErrorHandler error{ErrorType::SYNTACTIC, "Expected primary expression", tokens[current].line, tokens[current].col, tokens[current].snippet};
  throw error;
}
