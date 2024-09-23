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

#include <iostream>
#include <fstream>
#include <vector>
#include "../include/lexer.hpp"
#include "../include/parser.hpp"

#include "../include/interpreter.hpp"
#include "../include/compiler.hpp"


int main(int argc, char* argv[]) {
  // check if the user supplied a file name
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " <file_name> [-a] [-t] [-c]" << std::endl;
    std::cerr << "    -a: display the abstract syntax tree, TODO" << std::endl;
    std::cerr << "    -t: display the tokenized code" << std::endl;
    std::cerr << "    -c: compile the code" << std::endl;
    return 1;
  }

  // get the file name
  std::string file_name = argv[1];

  // check if the file name ends with .rss
  if (file_name.size() < 4 || file_name.substr(file_name.size() - 4) != ".csf") {
    std::cerr << "Error: file name must end with .csf" << std::endl;
    return 1;
  }

  // check for token and compile flags
  bool show_tokens = false;
  bool show_ast = false;
  bool compile = false;

  for (int i = 2; i < argc; ++i) {
    if (std::string(argv[i]) == "-t") {
      show_tokens = true;
    } else if (std::string(argv[i]) == "-c") {
      compile = true;
    } else if (std::string(argv[i]) == "-a") {
      show_ast = true;
    }
  }

  // read the code from the file
  std::ifstream file(file_name);
  if (!file.is_open()) {
    std::cerr << "Error: could not open file " << file_name << std::endl;
    return 1;
  }

  std::string code;
  std::string line;
  while (std::getline(file, line)) {
    code += line + "\n";
  }

  // if the file is empty, return error
  if (code.empty()) {
    std::cerr << "Error: file is empty" << std::endl;
    return 1;
  }

  // create a lexer
  Lexer lexer(code);

  // tokenize the code
  // std::vector<std::string> tokens = lexer.tokenize();
  std::vector<Token> tokens = lexer.tokenize();

  // display the tokenized code (as an array) if the user wants to
  if (show_tokens) {
    std::cout << "Tokenized code:" << std::endl;
    std::cout << "[";
    for (size_t i = 0; i < tokens.size(); i++) {
      std::cout << "{";
      /* std::cout << "\"type\": \"" << token_type_to_string(tokens[i].type) << "\", "; */
      /* std::cout << "\"line\": " << tokens[i].line << ", "; */
      /* std::cout << "\"column\": " << tokens[i].col << ", "; */
      /* std::cout << "\"value\": \"" << tokens[i].value << "\""; */
      token_to_string(tokens[i]);
      std::cout << "}";
      if (i < tokens.size() - 1) {
        std::cout << ", ";
      }
    }
    std::cout << "]" << std::endl;
  }

  // compile the code if the user wants to
  if (compile) {
    std::cerr << "Error: compilation is not implemented yet" << std::endl;
    // Compiler compiler(tokens);
    // compiler.compile();
  } else {
    std::cerr << "Error: parsing and interpreting are not implemented yet" << std::endl;
    // spawn a parser and parse the code
    // Parser parser(tokens);
    // std::vector<std::unique_ptr<ASTNode>> ast = parser.parse();

    // // print the AST if the user wants to
    // if (show_ast) {
    //   std::cout << "Abstract syntax tree:" << std::endl;
    //   std::cout << "[";
    //   for (size_t i = 0; i < ast.size(); i++) {
    //     // Capture the output of to_string() and print it
    //     std::cout << ast[i]->to_string();
    //     if (i < ast.size() - 1) {
    //       std::cout << ", ";
    //     }
    //   }
    //   std::cout << "]" << std::endl;
    // }

    // // interpret the code
    // Interpreter interpreter(ast);
    // interpreter.run();
  }

  return 0;
}
