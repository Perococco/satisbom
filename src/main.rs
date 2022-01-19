use maplit::hashmap;

use model::book::FilterableBook;

use crate::amount::{Amount, AmountF64, AmountRatio};
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
mod amount;
pub mod bag;
pub mod colors;


fn main() -> crate::error::Result<()> {

    let mut t = term::stdout().unwrap();

    let bom:Bom<AmountRatio> = optimize()?.into();

    bom.display(t.as_mut())?;

    t.reset()?;

    Ok(())
}

fn optimize() -> Result<Bom<AmountF64>> {
    let full_book = FullBook::create()?;

    let filter:fn(&Recipe) -> bool = |r| !r.alternate();

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
