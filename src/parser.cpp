/*
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: same as c++
- identifiers: [a-zA-Z_][a-zA-Z0-9_]*
- int: [0-9]+
- float: [0-9]+\.[0-9]+
- string: "[^"]*"
- bool: true, false
- operators: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=
- precedence:
    0 (highest): function call, scope (())
    1: unary operators (- (negative), ! (not), ++ (increment), -- (decrement))
    2: exponentiation (^)
    3: multiplication (*, /, %)
    4: addition (+, -)
    5: Comparison1 (<, <=, >, >=)
    6: Comparison2 (==, !=)
    7: Logical AND (&&)
    8: Logical OR (||)
    9 (lowest): assignment (=, +=, -=, *=, /=, %=)
*/

#include "../include/parser.hpp"
#include "../include/ast.hpp"
#include "../include/types.hpp"
#include <stdexcept>
#include <iostream>
#include <regex>

bool Parser::is_keyword(const Token& token) {
  return token.type == LET || token.type == DEL || token.type == IF || token.type == FOR || token.type == WHILE || token.type == BREAK || token.type == CONTINUE || token.type == RETURN || token.type == EXIT || token.type == FUNC || token.type == SWITCH || token.type == IMPORT;
}

bool Parser::is_assignment(const Token& token) {
  return token.type == ASSIGN || token.type == ADD_ASSIGN || token.type == SUB_ASSIGN || token.type == MUL_ASSIGN || token.type == DIV_ASSIGN || token.type == MOD_ASSIGN || token.type == POW_ASSIGN;
}

std::vector<std::unique_ptr<ASTNode>> Parser::parse() {
  while (current < tokens.size()) {
    std::unique_ptr<ASTNode> node = nullptr;
    if (is_keyword(tokens[current]) || is_assignment(tokens[current])) {
      node = parse_keyword();
    } else {
      node = parse_expression();
    }
    
    if (node) {
      ast.push_back(std::move(node));
    } else {
      std::cerr << "Error: Failed to create AST node: " << std::endl;
      token_to_string(tokens[current]);
    }
  }

  return std::move(ast);
}

std::unique_ptr<ASTNode> Parser::parse_keyword() {
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
    case STRUCT: return parse_struct();
    default: return parse_expression();
  }
}

std::unique_ptr<ASTNode> Parser::parse_import() {
  // import -> "<filepath>";
  current++; // consume "import"
  std::string rel_fpath = tokens[current].value;
  current++; // filepath
  current++; // consume ;
  return std::make_unique<ImportNode>(rel_fpath, tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_struct() {
  // struct <name> {
  //  <field_name>: type;
  //  <field_name>: type;
  // };
  current++; // consume struct
  std::string name = tokens[current].value;
  current++; // consume {
  std::vector<std::pair<std::string, Var_Types>> fields;
  std::string f_name;
  std::string f_type;
  do {
    f_name = tokens[current].value;
    current++; // consume field name
    current++; // consume :
    f_type = tokens[current].value;
    current++; //consume field type
    current++; // consume ;
    fields.push_back(std::make_pair(f_name, string_to_var_type(f_type)));
  } while(tokens[current].value != "}");
  current++; // consume "}"
  current++; // consume ;
}

std::unique_ptr<ASTNode> Parser::parse_let() {
  // let -> "let" IDENTIFIER: TYPE "=" expression ";"
  current++; // consume "let"
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  current++; // consume ":"
  std::string type = tokens[current].value;
  Var_Types var_type = string_to_var_type(type);
  current++; // consume TYPE
  if (tokens[current].value != "=") {
    throw std::runtime_error("Expected '=' after type in let statement");
  }
  current++; // consume "="
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after expression in let statement");
  }
  current++; // consume ";"
  return std::make_unique<LetNode>(identifier, var_type, std::move(expression), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_del() {
  // del -> "del" IDENTIFIER ";"
  current++; // consume "del"
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after identifier in del statement");
  }
  current++; // consume ";"
  return std::make_unique<DelNode>(identifier, tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_if() {
  // if -> "if" (expression) "{" ( keyword | expression )* "}" ( "elif" (expression) "{" ( keyword | expression )* "}" )* ( "else" "{" ( keyword | expression )* "}" )?
  current++; // consume "if"
  if (tokens[current].value != "(") {
    throw std::runtime_error("Expected '(' after 'if'");
  }
  current++; // consume "("
  std::unique_ptr<ASTNode> if_condition = parse_expression();
  if (tokens[current].value != ")") {
    throw std::runtime_error("Expected ')' after if condition");
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    throw std::runtime_error("Expected '{' after if condition");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> if_body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      throw std::runtime_error("Expected '}' after if body");
    }
    if (is_keyword(tokens[current])) {
      if_body.push_back(parse_keyword());
    } else {
      if_body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  // parse elifs, if any
  std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> elifs;
  while (current < tokens.size() && tokens[current].value == "elif") {
    current++; // consume "elif"
    if (tokens[current].value != "(") {
      throw std::runtime_error("Expected '(' after 'elif'");
    }
    current++; // consume "("
    std::unique_ptr<ASTNode> elif_condition = parse_expression();
    if (tokens[current].value != ")") {
      throw std::runtime_error("Expected ')' after elif condition");
    }
    current++; // consume ")"
    if (tokens[current].value != "{") {
      throw std::runtime_error("Expected '{' after elif condition");
    }
    current++; // consume "{"
    std::vector<std::unique_ptr<ASTNode>> elif_body;
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        throw std::runtime_error("Expected '}' after elif body");
      }
      if (is_keyword(tokens[current])) {
        elif_body.push_back(parse_keyword());
      } else {
        elif_body.push_back(parse_expression());
      }
    }
    current++; // consume "}"
    elifs.push_back(std::make_pair(std::move(elif_condition), std::move(elif_body)));
  }
  // parse else, if any
  std::vector<std::unique_ptr<ASTNode>> else_body;
  if (current < tokens.size() && tokens[current].value == "else") {
    current++; // consume "else"
    if (tokens[current].value != "{") {
      throw std::runtime_error("Expected '{' after 'else'");
    }
    current++; // consume "{"
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        throw std::runtime_error("Expected '}' after else body");
      }
      if (is_keyword(tokens[current])) {
        else_body.push_back(parse_keyword());
      } else {
        else_body.push_back(parse_expression());
      }
    }
    current++; // consume "}"
  }
  return std::make_unique<IEENode>(std::move(if_condition), std::move(if_body), std::move(elifs), std::move(else_body), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_for() {
  // for -> "for" (IDENTIFIER; expression; expression) "{" ( keyword | expression )* "}"
  current++; // consume "for"
  if (tokens[current].value != "(") {
    throw std::runtime_error("Expected '(' after 'for'");
  }
  current++; // consume "("
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after identifier in for statement");
  }
  current++; // consume ";"
  std::unique_ptr<ASTNode> condition = parse_expression();
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after condition in for statement");
  }
  current++; // consume ";"
  std::unique_ptr<ASTNode> increment = parse_expression();
  if (tokens[current].value != ")") {
    throw std::runtime_error("Expected ')' after increment in for statement");
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    throw std::runtime_error("Expected '{' after for statement");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      throw std::runtime_error("Expected '}' after for body");
    }
    if (is_keyword(tokens[current])) {
      body.push_back(parse_keyword());
    } else {
      body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  return std::make_unique<ForNode>(identifier,std::move(condition), std::move(increment), std::move(body), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_while() {
  // while -> "while" (expression) "{" ( keyword | expression )* "}"
  current++; // consume "while"
  if (tokens[current].value != "(") {
    throw std::runtime_error("Expected '(' after 'while'");
  }
  current++; // consume "("
  std::unique_ptr<ASTNode> condition = parse_expression();
  if (tokens[current].value != ")") {
    throw std::runtime_error("Expected ')' after condition in while statement");
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    throw std::runtime_error("Expected '{' after while condition");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      throw std::runtime_error("Expected '}' after while body");
    }
    if (is_keyword(tokens[current])) {
      body.push_back(parse_keyword());
    } else {
      body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  return std::make_unique<WhileNode>(std::move(condition), std::move(body), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_break() {
  // break -> "break" ";"
  current++; // consume "break"
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after 'break'");
  }
  current++; // consume ";"
  return std::make_unique<BreakNode>(tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_continue() {
  // continue -> "continue" ";"
  current++; // consume "continue"
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after 'continue'");
  }
  current++; // consume ";"
  return std::make_unique<ContinueNode>(tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_return() {
  // return -> "return" expression ";"
  current++; // consume "return"
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after expression in return statement");
  }
  current++; // consume ";"
  return std::make_unique<ReturnNode>(std::move(expression), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_exit() {
  // exit -> "exit" <expression> ";"
  current++; // consume "exit"
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current].value != ";") {
    throw std::runtime_error("Expected ';' after expression in exit statement");
  }
  current++; // consume ";"
  return std::make_unique<ExitNode>(std::move(expression), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_func() {
  // func -> "func" IDENTIFIER "(" (IDENTIFIER: TYPE ("," IDENTIFIER: TYPE)*)? "): " <return_type> "{" ( keyword | expression )* "}"
  // func IDENT(arg1: TYPE, arg2: TYPE): TYPE { ... }
  current++; // consume "func"
  std::string identifier = tokens[current].value;
  current++; // consume IDENTIFIER
  if (tokens[current].value != "(") {
    throw std::runtime_error("Expected '(' after function identifier");
  }
  current++; // consume "("
  std::vector<std::pair<std::string, Var_Types>> args;
  if (tokens[current].value != ")") {
    do {
      std::string arg_identifier = tokens[current].value;
      current++; // consume IDENTIFIER
      if (tokens[current].value != ":") {
        throw std::runtime_error("Expected ':' after argument identifier in function declaration");
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
    throw std::runtime_error("Expected ')' after function arguments");
  }
  current++; // consume ")"
  if (tokens[current].value != ":") {
    throw std::runtime_error("Expected ':' after function arguments closing parenthesis");
  }
  current++; // consume ":"
  Func_Types return_type = string_to_func_type(tokens[current].value);
  current++; // consume return type
  if (tokens[current].value != "{") {
    throw std::runtime_error("Expected '{' after function declaration");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> body;
  while (tokens[current].value != "}") {
    if (current >= tokens.size()) {
      throw std::runtime_error("Expected '}' after function body");
    }
    if (is_keyword(tokens[current])) {
      body.push_back(parse_keyword());
    } else {
      body.push_back(parse_expression());
    }
  }
  current++; // consume "}"
  return std::make_unique<FuncNode>(identifier, return_type, std::move(args), std::move(body), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_switch() {
  // switch -> "switch" (expression) "{" ( "case" (expression) "{" ( keyword | expression )* "}" )+ ( "default" "{" ( keyword | expression )* "}" )?
  current++; // consume "switch"
  if (tokens[current].value != "(") {
    throw std::runtime_error("Expected '(' after 'switch'");
  }
  current++; // consume "("
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current].value != ")") {
    throw std::runtime_error("Expected ')' after switch expression");
  }
  current++; // consume ")"
  if (tokens[current].value != "{") {
    throw std::runtime_error("Expected '{' after switch expression");
  }
  current++; // consume "{"
  std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> cases;
  while (current < tokens.size() && tokens[current].value == "case") {
    current++; // consume "case"
    if (tokens[current].value != "(") {
      throw std::runtime_error("Expected '(' after 'case'");
    }
    current++; // consume "("
    std::unique_ptr<ASTNode> case_expression = parse_expression();
    if (tokens[current].value != ")") {
      throw std::runtime_error("Expected ')' after case expression");
    }
    current++; // consume ")"
    if (tokens[current].value != "{") {
      throw std::runtime_error("Expected '{' after case expression");
    }
    current++; // consume "{"
    std::vector<std::unique_ptr<ASTNode>> case_body;
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        throw std::runtime_error("Expected '}' after case body");
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
  std::vector<std::unique_ptr<ASTNode>> default_body;
  if (current < tokens.size() && tokens[current].value == "default") {
    current++; // consume "default"
    if (tokens[current].value != "{") {
      throw std::runtime_error("Expected '{' after 'default'");
    }
    current++; // consume "{"
    while (tokens[current].value != "}") {
      if (current >= tokens.size()) {
        throw std::runtime_error("Expected '}' after default body");
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
  return std::make_unique<SCDNode>(std::move(expression), std::move(cases), std::move(default_body), tokens[current].line, tokens[current].col);
}

std::unique_ptr<ASTNode> Parser::parse_expression() {
  // expression -> assignment ;
  return parse_assignment();
}

std::unique_ptr<ASTNode> Parser::parse_assignment() {
  // assignment -> logical_or ( ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "^=") logical_or )* ;
  std::unique_ptr<ASTNode> node = parse_logical_or();

  while (current < tokens.size()) {
    if (tokens[current].value == "=" || tokens[current].value == "+=" || tokens[current].value == "-=" || tokens[current].value == "*=" || tokens[current].value == "/=" || tokens[current].value == "%=" || tokens[current].value == "^=") {
      std::string ident = tokens[current - 1].value;
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_logical_or();
      current++;
      /* node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col); */
      node = std::make_unique<SetNode>(op, ident, std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_logical_or() {
  // logical_or -> logical_and ( "||" logical_and )* ;
  std::unique_ptr<ASTNode> node = parse_logical_and();

  while (current < tokens.size()) {
    if (tokens[current].value == "||") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_logical_and();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_logical_and() {
  // logical_and -> equality ( "&&" equality )* ;
  std::unique_ptr<ASTNode> node = parse_equality();

  while (current < tokens.size()) {
    if (tokens[current].value == "&&") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_equality();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_equality() {
  // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
  std::unique_ptr<ASTNode> node = parse_comparison();

  while (current < tokens.size()) {
    if (tokens[current].value == "!=" || tokens[current].value == "==") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_comparison();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_comparison() {
  // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  std::unique_ptr<ASTNode> node = parse_term();

  while (current < tokens.size()) {
    if (tokens[current].value == ">" || tokens[current].value == ">=" || tokens[current].value == "<" || tokens[current].value == "<=") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_term();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_term() {
  // term -> factor ( ( "-" | "+" ) factor )* ;
  std::unique_ptr<ASTNode> node = parse_factor();

  while (current < tokens.size()) {
    if (tokens[current].value == "-" || tokens[current].value == "+") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_factor();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_factor() {
  // factor -> exponentiation ( ( "/" | "*" | "%" ) exponentiation )* ;
  std::unique_ptr<ASTNode> node = parse_exponentiation();

  while (current < tokens.size()) {
    if (tokens[current].value == "/" || tokens[current].value == "*" || tokens[current].value == "%") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_exponentiation();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_exponentiation() {
  // exponentiation -> unary | postfix ( "^" unary | postfix )* ;
  std::unique_ptr<ASTNode> node = parse_unary();

  while (current < tokens.size()) {
    if (tokens[current].value == "^") {
      std::string op = tokens[current].value;
      current++;
      std::unique_ptr<ASTNode> right = parse_unary();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right), tokens[current].line, tokens[current].col);
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_unary() {
  // unary -> ( "!" | "-" ) unary | primary ;
  if (tokens[current].value == "!" || tokens[current].value == "-") {
    std::string op = tokens[current].value;
    current++;
    std::unique_ptr<ASTNode> right = parse_unary();
    return std::make_unique<UnaryOpNode>(op, std::move(right), tokens[current].line, tokens[current].col);
  } else if (tokens[current].value == "++" || tokens[current].value == "--") {
    std::string ident = tokens[current - 1].value;
    std::string op = tokens[current].value;
    current++;
    // remove previous token
    ast.erase(ast.begin()+current-1);
    if (tokens[current].value == ";") {
      current++;
    }
    return std::make_unique<PostFixNode>(op, ident, tokens[current].line, tokens[current].col);
  } else {
    return parse_primary();
  }
}

std::unique_ptr<ASTNode> Parser::parse_primary() {
  // primary -> INT | Double | STRING | BOOL | "(" expression ")" | FuncCall
  if (tokens[current].value == "(") {
    current++;
    std::unique_ptr<ASTNode> node = parse_expression();
    if (tokens[current].value != ")") {
      throw std::runtime_error("Expected ')' after expression");
    }
    current++;
    return node;
  } else {
    std::string token = tokens[current].value;

    // Check for function call
    if (std::regex_match(token, std::regex("[a-zA-Z_][a-zA-Z0-9_]*")) && 
        current + 1 < tokens.size() && tokens[current + 1].value == "(") {
      current++; // consume the identifier
      current++; // consume the '('

      std::vector<std::unique_ptr<ASTNode>> args;
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
        throw std::runtime_error("Expected ')' after function arguments");
      }
      current++; // consume the ')'

      return std::make_unique<CallNode>(token, std::move(args), tokens[current].line, tokens[current].col);
    }

    // Check for INT
    if (std::regex_match(token, std::regex("[0-9]+"))) {
      current++;
      return std::make_unique<IntNode>(std::stoi(token), tokens[current].line, tokens[current].col);
    }

    // Check for FLOAT
    if (std::regex_match(token, std::regex("[0-9]+\\.[0-9]+"))) {
      current++;
      return std::make_unique<DoubleNode>(std::stof(token), tokens[current].line, tokens[current].col);
    }

    // Check for STRING
    if (std::regex_match(token, std::regex("\"[^\"]*\""))) {
      current++;
      return std::make_unique<StringNode>(token.substr(1, token.size() - 2), tokens[current].line, tokens[current].col);
    }

    // Check for Bool
    if (token == "true" || token == "false") {
      current++;
      return std::make_unique<BoolNode>(token == "true", tokens[current].line, tokens[current].col);
    }

    // If it's not a primary, assume it's a variable
    current++;
    return std::make_unique<VariableNode>(token, tokens[current].line, tokens[current].col);
  }

  throw std::runtime_error("Failed to parse primary");
}
