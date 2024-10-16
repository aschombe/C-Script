#pragma once
#include "ast.hpp"
#include "scope.hpp"
#include <filesystem>

class Interpreter {
  public:
  // Interpreter(const std::vector<std::unique_ptr<ASTNode>>& ast, const std::filesystem::path file);
  Interpreter(const std::vector<ASTNode*>& ast, const std::filesystem::path file);

  ~Interpreter() {
    for (auto node : ast) {
      delete node;
    }
  }

  // the main function to run the interpreter
  void run();

  private:
  // const std::vector<std::unique_ptr<ASTNode>>& ast;
  const std::vector<ASTNode*>& ast;
  Scope scope;
  std::filesystem::path ran_file;
  
  // Value interp(const std::unique_ptr<ASTNode>& node);
  Value interp(const ASTNode* node);
  Value interp_import(const ImportNode* node);
  Value interp_binop(const BinOpNode* node);
  Value interp_unaryop(const UnaryOpNode* node);
  Value interp_postfix(const PostFixNode* node);
  Value interp_let(const LetNode* node);
  Value interp_set(const SetNode* node);
  Value interp_del(const DelNode* node);  
  Value interp_iee(const IEENode* node);
  Value interp_for(const ForNode* node);
  Value interp_while(const WhileNode* node);
  Value interp_break(const BreakNode* node);
  Value interp_continue(const ContinueNode* node);
  Value interp_func(const FuncNode* node);
  Value interp_call(const CallNode* node);
  Value interp_return(const ReturnNode* node);
  Value interp_exit(const ExitNode* node);
  Value interp_scd(const SCDNode* node);
  Value interp_struct_def(const StructDef* node);
  Value interp_struct_init(const StructInit* node);
  Value interp_struct_access(const StructAccess* node);
};
