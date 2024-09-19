#pragma once
#include <string>


enum class Var_Types {
  INT,
  FLOAT,
  BOOL,
  STRING,
  UNKNOWN
};

inline std::string var_type_to_string(Var_Types type) {
  switch (type) {
    case Var_Types::INT:
      return "int";
    case Var_Types::FLOAT:
      return "float";
    case Var_Types::BOOL:
      return "bool";
    case Var_Types::STRING:
      return "string";
    case Var_Types::UNKNOWN:
      return "unknown";
  }

  return "unknown";
}

inline Var_Types string_to_var_type(const std::string& type) {
  if (type == "int") {
    return Var_Types::INT;
  } else if (type == "float") {
    return Var_Types::FLOAT;
  } else if (type == "bool") {
    return Var_Types::BOOL;
  } else if (type == "string") {
    return Var_Types::STRING;
  }

  return Var_Types::UNKNOWN;
}

enum class Func_Types {
  VOID,
  INT,
  FLOAT,
  BOOL,
  STRING,
  UNKNOWN
};

inline std::string func_type_to_string(Func_Types type) {
  switch (type) {
    case Func_Types::VOID:
      return "void";
    case Func_Types::INT:
      return "int";
    case Func_Types::FLOAT:
      return "float";
    case Func_Types::BOOL:
      return "bool";
    case Func_Types::STRING:
      return "string";
    case Func_Types::UNKNOWN:
      return "unknown";
  }

  return "unknown";
}

inline Func_Types string_to_func_type(const std::string& type) {
  if (type == "void") {
    return Func_Types::VOID;
  } else if (type == "int") {
    return Func_Types::INT;
  } else if (type == "float") {
    return Func_Types::FLOAT;
  } else if (type == "bool") {
    return Func_Types::BOOL;
  } else if (type == "string") {
    return Func_Types::STRING;
  }

  return Func_Types::UNKNOWN;
}
