#pragma once
#include "ast.hpp"

class Compiler {
  public:
  Compiler(const std::vector<ASTNode*>& ast);

  // main func to run compiler
  void run();

  private:
  // const std::vector<std::unique_ptr<ASTNode>>& ast;
  const std::vector<ASTNode*>& ast;
  std::string code;

  /* void compile(const std::unique_ptr<ASTNode>& node); */
  /* void compile_binop(const BinOpNode* node); */
  /* void compile_unaryop(const UnaryOpNode* node); */
  /* void compile_postfix(const PostFixNode* node); */
  /* void compile_let(const LetNode* node); */
  /* void compile_set(const SetNode* node); */
  /* void compile_del(const DelNode* node); */
  /* void compile_iee(const IEENode* node); */
  /* void compile_for(const ForNode* node); */
  /* void compile_while(const WhileNode* node); */
  /* void compile_break(const BreakNode* node); */
  /* void compile_continue(const ContinueNode* node); */
  /* void compile_func(const FuncNode* node); */
  /* void compile_call(const CallNode* node); */
  /* void compile_return(const ReturnNode* node); */
  /* void compile_exit(const ExitNode* node); */
  /* void compile_scd(const SCDNode* node); */
  /* void compile_struct_def(const StructDef* node); */
  /* void compile_struct_init(const StructInit* node); */
  /* void compile_struct_access(const StructAccess* node); */

  /* void compile_import(const ImportNode* node); */
};
