
#include <iostream>
#include "../include/lexer.hpp"

// test the lexer
int main() {
    // code to tokenize
    std::string code = "int factorial(n: int): int {\n"
                       "    if n == 0 {\n"
                       "        return 1;\n"
                       "    } else {\n"
                       "        return n * factorial(n - 1);\n"
                       "    }\n"
                       "}\n"
                       "int main(): int {\n"
                       "    return factorial(5);\n"
                       "}\n";
    
    std::cout << "Code to tokenize:" << std::endl;
    std::cout << code << std::endl;

    // create a lexer
    Lexer lexer(code);

    std::cout << "Tokenizing..." << std::endl;
    
    // tokenize the code
    std::vector<std::string> tokens = lexer.tokenize();
    
    std::cout << "Tokens:" << std::endl;

    // print the tokens
    for (std::string token : tokens) {
        std::cout << token << std::endl;
    }
    
    std::cout << "Done." << std::endl;
    return 0;
}