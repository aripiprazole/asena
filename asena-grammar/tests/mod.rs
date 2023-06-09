use asena_ast::{AsenaFile, Binary, Expr, Infix};
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

#[test]
fn simple() {
    let code = include_str!("simple.ase");
    let parser = Parser::from(Lexer::new(code)).run(asena_grammar::file);
    let file = AsenaFile::new(parser.build_tree().unwrap());

    println!("{file:#?}");
}

#[test]
fn sig_decl() {
    let code = "some_proof : 10 := 10 { proof }";
    let parser = Parser::from(Lexer::new(code)).run(asena_grammar::decl);

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn lam_expr() {
    let code = "\\a b -> c";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn sigma_expr() {
    let code = "(awa {})";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn unicode_expr() {
    let code = "a (@ 10)";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn qual_app_expr() {
    let code = "a b => a b";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));
    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::new(tree));
}

#[test]
fn app_expr() {
    let code = "a (@ b)";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));
    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::new(tree));
}

#[test]
fn qual_expr() {
    let code = "a => b";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));

    println!("{:#?}", parser.build_tree().data());
}

#[test]
fn group_expr() {
    let code = "(1 + 2)";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));
    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::new(tree));
}

#[test]
fn pi_expr() {
    let code = "(a: t) -> (b: t) -> a b";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));
    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::new(tree));
}

#[test]
fn anonymous_pi_expr() {
    let code = "m -> a -> m a";
    let parser = Parser::from(Lexer::new(code)).run(|p| asena_grammar::expr(p, Linebreak::Cont));
    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::new(tree));
}

#[test]
fn ask_stmt() {
    let code = "do { (Just a) <- findUser 105 }";
    let parser = Parser::from(Lexer::new(code)).run(asena_grammar::stmt);
    let tree = parser.build_tree().unwrap();

    println!("{:#?}", tree);
}
