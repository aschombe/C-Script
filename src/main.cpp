#include <iostream>
#include <fstream>
#include "../include/lexer.hpp"
#include "../include/compiler.hpp"

int main(int argc, char* argv[]) {
    // check if the user supplied a file name
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <file_name> [-c]" << std::endl;
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

    // check if the user wants to compile the code
    bool compile = false;
    if (argc == 3) {
        if (std::string(argv[2]) == "-c") {
            compile = true;
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
    if (code.size() == 0) {
        std::cerr << "Error: file is empty" << std::endl;
        return 1;
    }

    // create a lexer
    Lexer lexer(code);

    // tokenize the code
    std::vector<std::string> tokens = lexer.tokenize();

    // print the tokens as an array
    std::cout << "[";
    for (size_t i = 0; i < tokens.size(); i++) {
        std::cout << "\"" << tokens[i] << "\"";
        if (i < tokens.size() - 1) {
            std::cout << ", ";
        }
    }
    std::cout << "]" << std::endl;

    // compile the code if the user wants to
    if (compile) {
        std::cerr << "Error: compilation is not implemented yet" << std::endl;
        // Compiler compiler(tokens);
        // compiler.compile();
    } else {
        // interpret the code
        std::cerr << "Error: interpretation is not implemented yet" << std::endl;
        // Interpreter interpreter(tokens);
        // interpreter.interpret();
    }

    return 0;
}

// // test the lexer
// int main() {
//     // code to tokenize
//     std::string code = "int factorial(n: int): int {\n"
//                        "    if n == 0 {\n"
//                        "        return 1;\n"
//                        "    } else {\n"
//                        "        return n * factorial(n - 1);\n"
//                        "    }\n"
//                        "}\n"
//                        "int main(): int {\n"
//                        "    return factorial(5);\n"
//                        "}\n";
    
//     std::cout << "Code:" << std::endl;
//     std::cout << code << std::endl;

//     // create a lexer
//     Lexer lexer(code);
    
//     // tokenize the code
//     std::vector<std::string> tokens = lexer.tokenize();
    
//     std::cout << "Tokens:" << std::endl;

//     // print the tokens
//     for (std::string token : tokens) {
//         std::cout << token << std::endl;
//     }
    
//     return 0;
// }