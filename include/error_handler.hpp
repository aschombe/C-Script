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
  WARNING
};

class ErrorHandler : public std::exception {
  public:
  ErrorHandler() = default;
  ErrorHandler(ErrorType type, std::string message, const Token& token) : type(type), message(message), token(token) {}

  const char* what() const noexcept override {
    error_message = to_string(); 
    return error_message.c_str();
  }

  std::string to_string() const {
    std::string error_type;
    ErrorType type = this->type;
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
      case ErrorType::WARNING:
        error_type = "Warning";
        break;
    }
  
    std::string error_msg = message + "\n";
    
    if (type == ErrorType::WARNING) {
      error_msg += error_type + "warning at line " + std::to_string(token.line) + ", column " + std::to_string(token.col) + "\n";
    } else {
      error_msg += error_type + " error at line " + std::to_string(token.line) + ", column " + std::to_string(token.col) + "\n";
    }

    error_msg += token.snippet + "\n";
    error_msg += std::string(token.col - 1, ' ') + "^ " + message;
    
    return error_msg;
  }

  private:
  ErrorType type;
  std::string message;
  Token token;
  mutable std::string error_message;
};
