#pragma once

#include <string>
#include <iostream>
#include <exception>
#include "token.hpp"

enum class ErrorType {
  LEXICAL,
  SYNTACTIC,
  SEMANTIC,
  TYPE,
};

class ErrorHandler {
  public:
  ErrorHandler() = default;
  ErrorHandler(ErrorType type, std::string message, const Token& token) : type(type), message(message), token(token) {}

  std::string to_string() const {
    std::string error_type;
    switch (type) {
      case ErrorType::LEXICAL:
        error_type = "Lexical";
        break;
      case ErrorType::SYNTACTIC:
        error_type = "Syntactic";
        break;
      case ErrorType::SEMANTIC:
        error_type = "Semantic";
        break;
      case ErrorType::TYPE:
        error_type = "Type";
        break;
    }

    std::string error_msg = message + "\n" + error_type + " error at line " + std::to_string(token.line) + ", column " + std::to_string(token.col) + "\n";
    error_msg += token.snippet + "\n";
    error_msg += std::string(token.col - 1, ' ') + "^ " + message;
    return error_msg;
  }

};
