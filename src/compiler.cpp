#include "../include/compiler.hpp"
#include "../include/ast.hpp"
#include "../include/error_handler.hpp"

/* Compiler::Compiler(const std::vector<std::unique_ptr<ASTNode>>& ast) : ast(ast) {} */

// this will be a transpiler from AST to C
/* void Compiler::run() { */
/*   for (const auto& node : ast) { */
/*     compile(node); */
/*   } */
/* } */

/* void Compiler::compile(const std::unique_ptr<ASTNode>& node) { */
/*   switch(node->node_type()) { */
/*     case 0: // IntNode */
/*       code += std::to_string(static_cast<IntNode*>(node.get())->value); */
/*       break; */
/*     case 1: // DoubleNode */
/*       code += std::to_string(static_cast<DoubleNode*>(node.get())->value); */
/*       break; */
/*     case 2: // StringNode */
/*       code += "\"" + static_cast<StringNode*>(node.get())->value + "\""; */
/*       break; */
/*     case 3: // BoolNode */
/*       code += static_cast<BoolNode*>(node.get())->value ? "true" : "false"; */
/*       break; */
/*     case 4: // VariableNode */
/*       code += static_cast<VariableNode*>(node.get())->name; */
/*       break; */
/*     case 5: // BinOpNode */
/*       compile_binop(static_cast<BinOpNode*>(node.get())); */
/*       break; */
/*     case 6: // UnaryOpNode */
/*       compile_unaryop(static_cast<UnaryOpNode*>(node.get())); */
/*       break; */
/*     case 7: // PostFixNode */
/*       compile_postfix(static_cast<PostFixNode*>(node.get())); */
/*       break; */
/*     case 8: // LetNode */
/*       compile_let(static_cast<LetNode*>(node.get())); */
/*       break; */
/*     case 9: // SetNode */
/*       compile_set(static_cast<SetNode*>(node.get())); */
/*       break; */
/*     case 10: // DelNode */
/*       compile_del(static_cast<DelNode*>(node.get())); */
/*       break; */
/*     case 11: // IEENode */
/*       compile_iee(static_cast<IEENode*>(node.get())); */
/*       break; */
/*     case 12: // ForNode */
/*       compile_for(static_cast<ForNode*>(node.get())); */
/*       break; */
/*     case 13: // WhileNode */
/*       compile_while(static_cast<WhileNode*>(node.get())); */
/*       break; */
/*     case 14: // BreakNode */
/*       compile_break(static_cast<BreakNode*>(node.get())); */
/*       break; */
/*     case 15: // ContinueNode */
/*       compile_continue(static_cast<ContinueNode*>(node.get())); */
/*       break; */
/*     case 16: // FuncNode */
/*       compile_func(static_cast<FuncNode*>(node.get())); */
/*       break; */
/*     case 17: // CallNode */
/*       compile_call(static_cast<CallNode*>(node.get())); */
/*       break; */
/*     case 18: // ReturnNode */
/*       compile_return(static_cast<ReturnNode*>(node.get())); */
/*       break; */
/*     case 19: // ExitNode */
/*       compile_exit(static_cast<ExitNode*>(node.get())); */
/*       break; */
/*     case 20: // SCDNode */
/*       compile_scd(static_cast<SCDNode*>(node.get())); */
/*       break; */
/*     case 21: // StructDef */
/*       compile_struct_def(static_cast<StructDef*>(node.get())); */
/*       break; */
/*     case 22: // StructInit */
/*       compile_struct_init(static_cast<StructInit*>(node.get())); */
/*       break; */
/*     case 23: // StructAccess */
/*       compile_struct_access(static_cast<StructAccess*>(node.get())); */
/*       break; */
/*     case 24: // ImportNode */
/*       compile_import(static_cast<ImportNode*>(node.get())); */
/*       break; */
/*     default: */
/*       ErrorHandler error{ErrorType::SYNTACTIC, "Unknown node type", node->line, node->col, node->snippet}; */
/*       throw error; */
/*   } */
/* } */

/* void Compiler::compile_binop(const BinOpNode* node) { */
/*   compile(node->left); */
/*   code += " " + node->op + " "; */
/*   compile(node->right); */
/* } */
