
use crate::model::bom::Bom;
use crate::error::Result;
use crate::factory::Factory;
use crate::model::book::Book;
use crate::problem_input::ProblemInput;

pub fn solve(input: &ProblemInput, book: &dyn Book) -> Result<Bom> {
    let problem = Factory::create_problem(input,book)?;
    problem.solve()
}


