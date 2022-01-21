extern crate core;

use maplit::hashmap;

use model::book::FilterableBook;

use model::bom::Bom;
use crate::error::Result;
use crate::model::bom_printer::{AmountFormat, BomPrinter};
use crate::model::dot::Graph;
use crate::model::full_book::FullBook;
use crate::model::recipe::Recipe;
use crate::problem_input::ProblemInput;
use crate::solver::solve;

pub mod model;
mod problem_input;
mod solver;
pub mod error;
pub mod factory;
pub mod problem;
pub mod colors;
mod constants;


fn main() -> crate::error::Result<()> {

    let bom:Bom = optimize()?;

    let mut printer = BomPrinter::with_term( AmountFormat::Ratio);

    bom.display(&mut printer)?;


    let graph = Graph::new(&bom);

    use std::fs::File;
    let mut f = File::create("example2.dot").unwrap();

    dot::render(&graph, &mut f);


    Ok(())
}

fn optimize() -> Result<Bom> {
    let full_book = FullBook::create()?;

    let filter:fn(&Recipe) -> bool = |r| true;//r.alternate() ;

    let book = full_book.filter(&filter)?;



    let input = ProblemInput{
        target_items: hashmap! {
             "rubber".to_string() => 30,
            "plastic".to_string() => 30,
        },
        available_items: hashmap! {
        }};


    let bom = solve(&input, &book).unwrap();

    Ok(bom)
}
