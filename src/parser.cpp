#include "../include/parser.hpp"
#include "../include/ast.hpp"
#include "../include/types.hpp"
#include <stdexcept>
#include <iostream>
#include <regex>

bool Parser::is_keyword(const std::string& token) {
    return token == "let" || token == "set" || token == "del" || token == "if" || token == "for" || token == "while" || token == "break" || token == "continue" || token == "return" || token == "exit" || token == "func" || token == "switch";
}

std::vector<std::unique_ptr<ASTNode>> Parser::parse() {
    while (current < tokens.size()) {
        std::unique_ptr<ASTNode> node = nullptr;
        if (is_keyword(tokens[current])) {
            node = parse_keyword();
        } else {
            node = parse_expression();
        }
        
        if (node) {
            ast.push_back(std::move(node));
        } else {
            std::cerr << "Error: Failed to create AST node" << std::endl;
        }
    }

    // Now return the AST, using std::move
    return std::move(ast); 
}

std::unique_ptr<ASTNode> Parser::parse_keyword() {
  // check for any of the keywords
  if (tokens[current] == "let") {
    return parse_let();
  } else if (tokens[current] == "set") {
    return parse_set();
  } else if (tokens[current] == "del") {
    return parse_del();
  } else if (tokens[current] == "if") {
    return parse_if();
  } else if (tokens[current] == "for") {
    return parse_for();
  } else if (tokens[current] == "while") {
    return parse_while();
  } else if (tokens[current] == "break") {
    return parse_break();
  } else if (tokens[current] == "continue") {
    return parse_continue();
  } else if (tokens[current] == "return") {
    return parse_return();
  } else if (tokens[current] == "exit") {
    return parse_exit();
  } else if (tokens[current] == "func") {
    return parse_func();
  } else if (tokens[current] == "switch") {
    return parse_switch();
  } else {
    return parse_expression();
  }
}

std::unique_ptr<ASTNode> Parser::parse_let() {
  // let -> "let" IDENTIFIER: TYPE "=" expression ";"
  current++; // consume "let"
  std::string identifier = tokens[current];
  current++; // consume IDENTIFIER
  current++; // consume ":"
  std::string type = tokens[current];
  current++; // consume TYPE
  if (tokens[current] != "=") {
    throw std::runtime_error("Expected '=' after type in let statement");
  }
  current++; // consume "="
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after expression in let statement");
  }
  current++; // consume ";"
  return std::make_unique<LetNode>(identifier, string_to_var_type(type), std::move(expression)); 
}

std::unique_ptr<ASTNode> Parser::parse_set() {
  // set -> IDENTIFIER ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "^=" ) expression ";"
  std::string identifier = tokens[current];
  current++; // consume IDENTIFIER
  if (tokens[current] != "=" && tokens[current] != "+=" && tokens[current] != "-=" && tokens[current] != "*=" && tokens[current] != "/=" && tokens[current] != "%=" && tokens[current] != "^=") {
    throw std::runtime_error("Expected assignment operator after identifier in set statement");
  }
  std::string op = tokens[current];
  current++; // consume assignment operator
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after expression in set statement");
  }
  current++; // consume ";"
  return std::make_unique<SetNode>(op, std::make_unique<VariableNode>(identifier), std::move(expression));
}

std::unique_ptr<ASTNode> Parser::parse_del() {
  // del -> "del" IDENTIFIER ";"
  current++; // consume "del"
  std::string identifier = tokens[current];
  current++; // consume IDENTIFIER
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after identifier in del statement");
  }
  current++; // consume ";"
  return std::make_unique<DelNode>(identifier);
}

std::unique_ptr<ASTNode> Parser::parse_if() {
  // if -> "if" (expression) "{" ( keyword | expression )* "}" ( "elif" (expression) "{" ( keyword | expression )* "}" )* ( "else" "{" ( keyword | expression )* "}" )?
  current++; // consume "if"
  if (tokens[current] != "(") {
    throw std::runtime_error("Expected '(' after 'if'");
  }
  current++; // consume "("
  std::unique_ptr<ASTNode> if_condition = parse_expression();
  // error is thrown in here because it thinks the next token is the last character of the condition not the ')'
  if (tokens[current] != ")") {
    throw std::runtime_error("Expected ')' after if condition");
  }
  current++; // consume ")"
  if (tokens[current] != "{") {
    throw std::runtime_error("Expected '{' after if condition");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> if_body;
  while (tokens[current] != "}") {
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
  while (current < tokens.size() && tokens[current] == "elif") {
    current++; // consume "elif"
    if (tokens[current] != "(") {
      throw std::runtime_error("Expected '(' after 'elif'");
    }
    current++; // consume "("
    std::unique_ptr<ASTNode> elif_condition = parse_expression();
    if (tokens[current] != ")") {
      throw std::runtime_error("Expected ')' after elif condition");
    }
    current++; // consume ")"
    if (tokens[current] != "{") {
      throw std::runtime_error("Expected '{' after elif condition");
    }
    current++; // consume "{"
    std::vector<std::unique_ptr<ASTNode>> elif_body;
    while (tokens[current] != "}") {
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
  if (current < tokens.size() && tokens[current] == "else") {
    current++; // consume "else"
    if (tokens[current] != "{") {
      throw std::runtime_error("Expected '{' after 'else'");
    }
    current++; // consume "{"
    while (tokens[current] != "}") {
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
  return std::make_unique<IEENode>(std::move(if_condition), std::move(if_body), std::move(elifs), std::move(else_body));
}

std::unique_ptr<ASTNode> Parser::parse_for() {
  // for -> "for" (IDENTIFIER; expression; expression) "{" ( keyword | expression )* "}"
  current++; // consume "for"
  if (tokens[current] != "(") {
    throw std::runtime_error("Expected '(' after 'for'");
  }
  current++; // consume "("
  std::string identifier = tokens[current];
  current++; // consume IDENTIFIER
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after identifier in for statement");
  }
  current++; // consume ";"
  std::unique_ptr<ASTNode> condition = parse_expression();
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after condition in for statement");
  }
  current++; // consume ";"
  std::unique_ptr<ASTNode> increment = parse_expression();
  if (tokens[current] != ")") {
    throw std::runtime_error("Expected ')' after increment in for statement");
  }
  current++; // consume ")"
  if (tokens[current] != "{") {
    throw std::runtime_error("Expected '{' after for statement");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> body;
  while (tokens[current] != "}") {
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
  return std::make_unique<ForNode>(identifier, std::move(condition), std::move(increment), std::move(body));
}

std::unique_ptr<ASTNode> Parser::parse_while() {
  // while -> "while" (expression) "{" ( keyword | expression )* "}"
  current++; // consume "while"
  if (tokens[current] != "(") {
    throw std::runtime_error("Expected '(' after 'while'");
  }
  current++; // consume "("
  std::unique_ptr<ASTNode> condition = parse_expression();
  if (tokens[current] != ")") {
    throw std::runtime_error("Expected ')' after condition in while statement");
  }
  current++; // consume ")"
  if (tokens[current] != "{") {
    throw std::runtime_error("Expected '{' after while condition");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> body;
  while (tokens[current] != "}") {
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
  return std::make_unique<WhileNode>(std::move(condition), std::move(body));
}

std::unique_ptr<ASTNode> Parser::parse_break() {
  // break -> "break" ";"
  current++; // consume "break"
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after 'break'");
  }
  current++; // consume ";"
  return std::make_unique<BreakNode>();
}

std::unique_ptr<ASTNode> Parser::parse_continue() {
  // continue -> "continue" ";"
  current++; // consume "continue"
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after 'continue'");
  }
  current++; // consume ";"
  return std::make_unique<ContinueNode>();
}

std::unique_ptr<ASTNode> Parser::parse_return() {
  // return -> "return" expression ";"
  current++; // consume "return"
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after expression in return statement");
  }
  current++; // consume ";"
  return std::make_unique<ReturnNode>(std::move(expression));
}

std::unique_ptr<ASTNode> Parser::parse_exit() {
  // exit -> "exit" <expression> ";"
  current++; // consume "exit"
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current] != ";") {
    throw std::runtime_error("Expected ';' after expression in exit statement");
  }
  current++; // consume ";"
  return std::make_unique<ExitNode>(std::move(expression));
}

std::unique_ptr<ASTNode> Parser::parse_func() {
  // func -> "func" IDENTIFIER "(" (IDENTIFIER: TYPE ("," IDENTIFIER: TYPE)*)? "): " <return_type> "{" ( keyword | expression )* "}"
  current++; // consume "func"
  std::string identifier = tokens[current];
  current++; // consume IDENTIFIER
  if (tokens[current] != "(") {
    throw std::runtime_error("Expected '(' after function identifier");
  }
  current++; // consume "("
  std::vector<std::pair<std::string, Var_Types>> args;
  if (tokens[current] != ")") {
    do {
      std::string arg_identifier = tokens[current];
      current++; // consume IDENTIFIER
      if (tokens[current] != ":") {
        throw std::runtime_error("Expected ':' after argument identifier in function declaration");
      }
      current++; // consume ":"
      std::string arg_type = tokens[current];
      current++; // consume TYPE
      args.push_back(std::make_pair(arg_identifier, string_to_var_type(arg_type)));
      if (tokens[current] == ",") {
        current++; // consume ","
      } else {
        break;
      }
    } while (current < tokens.size());
  }
  if (tokens[current] != ")") {
    throw std::runtime_error("Expected ')' after function arguments");
  }
  current++; // consume ")"
  if (tokens[current] != "):") {
    throw std::runtime_error("Expected '):' after function arguments");
  }
  current++; // consume "):"
  Func_Types return_type = string_to_func_type(tokens[current]);
  current++; // consume return type
  if (tokens[current] != "{") {
    throw std::runtime_error("Expected '{' after function declaration");
  }
  current++; // consume "{"
  std::vector<std::unique_ptr<ASTNode>> body;
  while (tokens[current] != "}") {
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
  return std::make_unique<FuncNode>(identifier, return_type, std::move(args), std::move(body));
}

std::unique_ptr<ASTNode> Parser::parse_switch() {
  /*
  switch (expression) {
    (case (expression) {
      ( keyword | expression )*
    })+
    (default {
      ( keyword | expression )*
    })?
  }
  */
  current++; // consume "switch"
  if (tokens[current] != "(") {
    throw std::runtime_error("Expected '(' after 'switch'");
  }
  current++; // consume "("
  std::unique_ptr<ASTNode> expression = parse_expression();
  if (tokens[current] != ")") {
    throw std::runtime_error("Expected ')' after switch expression");
  }
  current++; // consume ")"
  if (tokens[current] != "{") {
    throw std::runtime_error("Expected '{' after switch expression");
  }
  current++; // consume "{"
  std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> cases;
  while (current < tokens.size() && tokens[current] == "case") {
    current++; // consume "case"
    if (tokens[current] != "(") {
      throw std::runtime_error("Expected '(' after 'case'");
    }
    current++; // consume "("
    std::unique_ptr<ASTNode> case_expression = parse_expression();
    if (tokens[current] != ")") {
      throw std::runtime_error("Expected ')' after case expression");
    }
    current++; // consume ")"
    if (tokens[current] != "{") {
      throw std::runtime_error("Expected '{' after case expression");
    }
    current++; // consume "{"
    std::vector<std::unique_ptr<ASTNode>> case_body;
    while (tokens[current] != "}") {
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
  if (current < tokens.size() && tokens[current] == "default") {
    current++; // consume "default"
    if (tokens[current] != "{") {
      throw std::runtime_error("Expected '{' after 'default'");
    }
    current++; // consume "{"
    while (tokens[current] != "}") {
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
  return std::make_unique<SCDNode>(std::move(expression), std::move(cases), std::move(default_body));
}

std::unique_ptr<ASTNode> Parser::parse_expression() {
  // expression → assignment ;
  return parse_assignment();
}

std::unique_ptr<ASTNode> Parser::parse_assignment() {
  // assignment → logical_or ( ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" ) logical_or )* ;
  std::unique_ptr<ASTNode> node = parse_logical_or();

  while (current < tokens.size()) {
    if (tokens[current] == "=" || tokens[current] == "+=" || tokens[current] == "-=" || tokens[current] == "*=" || tokens[current] == "/=" || tokens[current] == "%=") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_logical_or();
      node = std::make_unique<SetNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_logical_or() {
  // logical_or → logical_and ( "||" logical_and )* ;
  std::unique_ptr<ASTNode> node = parse_logical_and();

  while (current < tokens.size()) {
    if (tokens[current] == "||") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_logical_and();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_logical_and() {
  // logical_and → equality ( "&&" equality )* ;
  std::unique_ptr<ASTNode> node = parse_equality();

  while (current < tokens.size()) {
    if (tokens[current] == "&&") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_equality();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_equality() {
  // equality → comparison ( ( "!=" | "==" ) comparison )* ;
  std::unique_ptr<ASTNode> node = parse_comparison();

  while (current < tokens.size()) {
    if (tokens[current] == "!=" || tokens[current] == "==") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_comparison();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_comparison() {
  // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  std::unique_ptr<ASTNode> node = parse_term();

  while (current < tokens.size()) {
    if (tokens[current] == ">" || tokens[current] == ">=" || tokens[current] == "<" || tokens[current] == "<=") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_term();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_term() {
  // term → factor ( ( "-" | "+" ) factor )* ;
  std::unique_ptr<ASTNode> node = parse_factor();

  while (current < tokens.size()) {
    if (tokens[current] == "-" || tokens[current] == "+") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_factor();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_factor() {
  // factor → exponentiation ( ( "/" | "*" | "%" ) exponentiation )* ;
  std::unique_ptr<ASTNode> node = parse_exponentiation();

  while (current < tokens.size()) {
    if (tokens[current] == "/" || tokens[current] == "*" || tokens[current] == "%") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_exponentiation();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_exponentiation() {
  // exponentiation → unary ( "^" unary )* ;
  std::unique_ptr<ASTNode> node = parse_unary();

  while (current < tokens.size()) {
    if (tokens[current] == "^") {
      std::string op = tokens[current];
      current++;
      std::unique_ptr<ASTNode> right = parse_unary();
      node = std::make_unique<BinOpNode>(op, std::move(node), std::move(right));
    } else {
      break;
    }
  }

  return node;
}

std::unique_ptr<ASTNode> Parser::parse_unary() {
  // unary → ( "!" | "-" ) unary | primary ;
  if (tokens[current] == "!" || tokens[current] == "-") {
    std::string op = tokens[current];
    current++;
    std::unique_ptr<ASTNode> right = parse_unary();
    return std::make_unique<UnaryOpNode>(op, std::move(right));
  } else {
    return parse_primary();
  }
}

std::unique_ptr<ASTNode> Parser::parse_primary() {
  // primary → INT | FLOAT | STRING | BOOL | "(" expression ")" | FuncCall
  if (tokens[current] == "(") {
    current++;
    std::unique_ptr<ASTNode> node = parse_expression();
    if (tokens[current] != ")") {
      throw std::runtime_error("Expected ')' after expression");
    }
    current++;
    return node;
  } else {
    std::string token = tokens[current];

    // Check for function call
    if (std::regex_match(token, std::regex("[a-zA-Z_][a-zA-Z0-9_]*")) && 
        current + 1 < tokens.size() && tokens[current + 1] == "(") {
      current++; // consume the identifier
      current++; // consume the '('

      std::vector<std::unique_ptr<ASTNode>> args;
      // Optionally parse arguments if they exist
      if (tokens[current] != ")") {
        do {
          args.push_back(parse_expression());
          if (tokens[current] == ",") {
            current++; // consume ','
          } else {
            break;
          }
        } while (current < tokens.size());
      }
      
      if (tokens[current] != ")") {
        throw std::runtime_error("Expected ')' after function arguments");
      }
      current++; // consume the ')'

      return std::make_unique<CallNode>(token, std::move(args));
    }

    // Check for INT
    if (std::regex_match(token, std::regex("[0-9]+"))) {
      current++;
      return std::make_unique<IntNode>(std::stoi(token));
    }

    // Check for FLOAT
    if (std::regex_match(token, std::regex("[0-9]+\\.[0-9]+"))) {
      current++;
      return std::make_unique<FloatNode>(std::stof(token));
    }

    // Check for STRING
    if (std::regex_match(token, std::regex("\"[^\"]*\""))) {
      current++;
      return std::make_unique<StringNode>(token.substr(1, token.size() - 2));
    }

    // Check for BOOL
    if (token == "true" || token == "false") {
      current++;
      return std::make_unique<BoolNode>(token == "true");
    }

    // If it's not a primary, assume it's a variable
    current++;
    return std::make_unique<VariableNode>(token);
  }
}
