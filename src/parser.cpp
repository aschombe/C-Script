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
    1: unary operators (- (negative), ! (not))
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
#include <stdexcept>
#include <regex>

std::unique_ptr<ASTNode> Parser::parse() {
  return parse_expression();
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
    if (token.front() == '"' && token.back() == '"') {
      current++;
      return std::make_unique<StringNode>(token.substr(1, token.size() - 2)); // Remove quotes
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
