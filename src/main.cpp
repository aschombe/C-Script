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
    if (file_name.size() < 4 || file_name.substr(file_name.size() - 4) != ".rss") {
        std::cerr << "Error: file name must end with .rss" << std::endl;
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
    std::vector<std::string> tokens = lexer.tokenize();

    // display the tokenized code (as an array) if the user wants to
    if (show_tokens) {
        std::cout << "[";
        for (size_t i = 0; i < tokens.size(); i++) {
            std::cout << "\"" << tokens[i] << "\"";
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
        // spawn a parser and parse the code
        Parser parser(tokens);
        std::unique_ptr<ASTNode> ast = parser.parse();

        // print the AST if the user wants to
        if (show_ast) {
            // std::cout << "AST to string not implemented yet" << std::endl;
            std::cout << ast->to_string() << std::endl;
        }

        // interpret the code
        // std::cerr << "Error: interpretation is not implemented yet" << std::endl;
        // Interpreter interpreter(tokens);
        // interpreter.interpret();
    }

    return 0;
}