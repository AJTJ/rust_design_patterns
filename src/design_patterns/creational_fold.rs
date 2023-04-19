/*
https://rust-unofficial.github.io/patterns/patterns/creational/fold.html
What it does:
- Run an algorithm over each item in a collection of data to create a new item, thus creating a whole new collection.
- Definitely more like a "map" pattern
- the fold pattern allows us to separate traversal of a data structure from the operations performed to each node.
 */

mod ast {
    pub enum Stmt {
        Expr(Box<Expr>),
        Let(Box<Name>, Box<Expr>),
    }

    pub struct Name {
        pub value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

mod fold {
    use super::ast::*;

    pub trait Folder {
        // a leaf node returns the node itself
        fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
            n
        }
        // Create a new inner node by folding its children
        fn fold_stmt(&mut self, s: Box<Stmt>) -> Box<Stmt> {
            match *s {
                Stmt::Expr(e) => Box::new(Stmt::Expr(self.fold_expr(e))),
                Stmt::Let(n, e) => Box::new(Stmt::Let(self.fold_name(n), self.fold_expr(e))),
            }
        }
        fn fold_expr(&mut self, e: Box<Expr>) -> Box<Expr> {
            unimplemented!()
        }
    }
}

use ast::*;
use fold::*;

struct Renamer;
impl Folder for Renamer {
    fn fold_name(&mut self, n: Box<Name>) -> Box<Name> {
        Box::new(Name {
            value: "foo".to_owned(),
        })
    }

    // use default method for other nodes
}
