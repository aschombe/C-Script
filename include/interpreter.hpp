#pragma once
#include "ast.hpp"
#include "scope.hpp"

class Interpreter {
  public:
  Interpreter(const std::vector<std::unique_ptr<ASTNode>>& ast);

  // the main function to run the interpreter
  void run();

  private:
  const std::vector<std::unique_ptr<ASTNode>>& ast;
  std::vector<Scope> scopes;

  Value interp(const std::unique_ptr<ASTNode>& node); 
  /* Value interp_variable(const std::shared_ptr<VariableNode>& node); */
  /* Value interp_binop(const std::shared_ptr<BinOp>& node); */
  /* Value interp_unaryop(const std::shared_ptr<UnaryOp>& node); */
  /* Value interp_let(const std::shared_ptr<Let>& node); */
  /* Value interp_set(const std::shared_ptr<Set>& node); */
  /* Value interp_del(const std::shared_ptr<Del>& node); */
  /* Value interp_iee(const std::shared_ptr<IEE>& node); */
  /* Value interp_for(const std::shared_ptr<For>& node); */
  /* Value interp_while(const std::shared_ptr<While>& node); */
  /* Value interp_break(const std::shared_ptr<Break>& node); */
  /* Value interp_continue(const std::shared_ptr<Continue>& node); */
  /* Value interp_func(const std::shared_ptr<Func>& node); */
  /* Value interp_call(const std::shared_ptr<Call>& node); */
  /* Value interp_return(const std::shared_ptr<Return>& node); */
  /* Value interp_exit(const std::shared_ptr<Exit>& node); */
  /* Value interp_scd(const std::shared_ptr<SCD>& node); */
};
