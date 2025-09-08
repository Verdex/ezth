
use crate::data::*;

use super::lexer;

/*
    def sym = expr

    match sym = ?

    pat sym = ? 


    Expr
    fun (sym_list) => expr
    sym
    expr.sym
    expr |> call_expr
    expr |index_pattern> call_expr
    (expr)
    expr(expr_list)

    Pattern
    data (exact)
    _
    a
    $a
    cons(pattern_list)
    and
    or
    [| pattern_list |]
    {| next_pattern_list |}
    predicate
    match_with ?


*/


// [fn(input) -> intermedia result]

// TODO need different error
pub fn parse(input : &str) -> Result<(Vec<ExprOrDef>, ?), ()> {

    let output = lexer::lex(input); 





    todo!()
}
