use maplit::hashmap;

use model::book::FilterableBook;

use crate::bom::Bom;
use crate::error::Result;
use crate::model::full_book::FullBook;
use crate::model::recipe::Recipe;
use crate::problem_input::ProblemInput;
use crate::solver::solve;

pub mod model;
mod problem_input;
mod bom;
mod solver;
pub mod error;
pub mod factory;
pub mod production;
pub mod bag;
pub mod colors;
mod constants;


fn main() -> crate::error::Result<()> {

    let mut t = term::stdout().unwrap();

    let bom:Bom = optimize()?;

    bom.display(t.as_mut())?;

    t.reset()?;

    Ok(())
}

fn optimize() -> Result<Bom> {
    let full_book = FullBook::create()?;

    let filter:fn(&Recipe) -> bool = |r| true || !r.alternate();

    let book = full_book.filter(&filter)?;



    let input = ProblemInput{
        requested_output: hashmap! {
            "plastic".to_string() => 0,
            "rubber".to_string() => 0,
            "turbofuel".to_string() => 60
        },
        available_items: hashmap! {
        }};


    let bom = solve(&input, &book).unwrap();

    Ok(bom)
}
