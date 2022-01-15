use maplit::hashmap;
use crate::book::FilterableBook;
use crate::solver::solve;
use crate::error::Result;
use crate::model::full_book::FullBook;
use crate::model::recipe::Recipe;
use crate::problem_input::ProblemInput;

pub mod model;
mod book;
mod problem_input;
mod problem_output;
mod solver;
pub mod error;
pub mod factory;
pub mod production;


fn main() -> Result<()> {
    let full_book = FullBook::create()?;

    let filter:fn(&Recipe) -> bool = |r| !r.alternate();

    let book = full_book.filter(&filter)?;



    let input = ProblemInput{
        requested_output: hashmap! {
            "iron_plate".to_string() => 60,
            "iron_rod".to_string() => 30
        },
        available_items: hashmap! {
            "iron_ingot".to_string() => 240
        }};


    let result = solve(&input, &book).unwrap();
    println!("{}", result);

    Ok(())
}
