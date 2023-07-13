use asena_ast::{Binary, Infix};
use asena_grammar::Linebreak;
use asena_leaf::ast::Node;
use asena_lexer::Lexer;
use asena_parser::Parser;

#[test]
fn it_works() {
    let code = "53 + 75 + 42";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));
    let infix = Infix::new(parser.build_tree().unwrap());

    let lhs = infix.lhs();
    let rhs = infix.find_rhs().as_new_node();

    infix.set_rhs(lhs);
    infix.set_lhs(rhs.as_leaf());

    println!("{:#?}", infix);
}
