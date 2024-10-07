#pragma once

#include "types.hpp"
#include <vector>
#include <algorithm>

// accepts a list of compatible types as the first argument, and the actual type as the second argument
inline bool check_type(const std::vector<Var_Types>& types, Var_Types type) {
  for (const auto& t : types) {
    if (t == type || can_convert(type, t)) {
      return true;
    }
  }

  return false;
}

// checks type conversions when type checking
inline can_convert(Var_Types from, Var_Types to) {
  static const std::unordered_map<Var_Types, std::vector<Var_Types>> conversions = {
    {Var_Types::INT, {Var_Types::FLOAT}},
    {Var_Types::FLOAT, {Var_Types::INT}}
  };

  if (from == to) return true;

  auto it = conversions.find(from);
  if (it != conversions.end()) {
    return std::find(it->second.begin(), it->second.end(), to) != it->second.end();
  }

  return false;
}
