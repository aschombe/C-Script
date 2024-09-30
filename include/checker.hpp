#pragma once

/* #include "ast.hpp" */
#include "types.hpp"
#include <vector>

// accepts a list of compatible types as the first argument, and the actual type as the second argument
inline bool check_type(const std::vector<Var_Types>& types, Var_Types type) {
  for (const auto& t : types) {
    if (t == type) {
      return true;
    }
  }

  return false;
}
