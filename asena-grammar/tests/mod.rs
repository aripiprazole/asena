use asena_ast::{AsenaFile, Binary, Expr, Infix};
use asena_leaf::ast::Leaf;
use asena_lexer::Lexer;
use asena_parser::Parser;

#[test]
fn it_works() {
    let code = "53 + 75 + 42";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    let infix = Infix::new(parser.build_tree().unwrap());

    let lhs = infix.lhs();
    let rhs = infix.rhs().as_new_node();

    infix.rhs().set(lhs);
    infix.lhs().set(rhs);

    println!("{:#?}", infix);
}

#[test]
fn simple() {
    let code = include_str!("simple.ase");

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::file(&mut parser);
    let file = parser.build_tree().unwrap();
    println!("{:#?}", file);

    println!("{:#?}", AsenaFile::new(file));
}

#[test]
fn sig_decl() {
    let code = "some_proof : 10 := 10 { proof }";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::decl(&mut parser);

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn lam_expr() {
    let code = "\\a b -> c";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn sigma_expr() {
    let code = "(awa {})";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn unicode_expr() {
    let code = "Π (d: t) -> (e Π =>)";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    println!("{:#?}", parser.build_tree().unwrap());
}

#[test]
fn qual_app_expr() {
    let code = "a b => a b";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::make(tree));
}

#[test]
fn app_expr() {
    let code = "a (@ b)";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::make(tree));
}

#[test]
fn qual_expr() {
    let code = "a => b";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    println!("{:#?}", parser.build_tree().data());
}

#[test]
fn group_expr() {
    let code = "(1 + 2)";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::make(tree));
}

#[test]
fn pi_expr() {
    let code = "(a: t) -> (b: t) -> a b";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::make(tree));
}

#[test]
fn anonymous_pi_expr() {
    let code = "m -> a -> m a";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::expr(&mut parser);

    let tree = parser.build_tree().unwrap();

    println!("{:#?}", Expr::make(tree));
}

#[test]
fn ask_stmt() {
    let code = "do { (Just a) <- findUser 105 }";

    let mut parser = Parser::from(Lexer::new(code));
    asena_grammar::stmt(&mut parser);

    let tree = parser.build_tree().unwrap();

    println!("{:#?}", tree);
}
