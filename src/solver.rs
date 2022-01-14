use good_lp::{default_solver, Expression, SolverModel, ProblemVariables, Variable, variable, Solution};
use crate::book::Book;
use crate::problem_input::ProblemInput;
use crate::problem_output::ProblemOutput;
use crate::error::Result;
use crate::factory::Factory;
use crate::input::recipe::Recipe;

pub fn solve<'a>(input: &ProblemInput, book: &'a dyn Book) -> Result<ProblemOutput<'a>> {
    let mut variables = ProblemVariables::new();

    let nb_recipes = book.number_of_recipes();

    let recipe_amount: Vec<Variable> = variables.add_vector(variable().min(0), nb_recipes);


    let production = Factory::compute_production(book, &input, &recipe_amount)?;


    let objective: Expression = production.objective();
    let constraints = production.compute_constraints();

    let variables = variables.maximise(Expression::from_other_affine(&objective)).using(default_solver);

    let variables = constraints.into_iter().fold(variables, |variables, c| variables.with(c));


    let result = variables.solve()?;


    let o = objective.eval_with(&result);
    println!("{}", o);

    let used_recipes = recipe_amount
        .into_iter()
        .enumerate()
        .map(|(i, variable)| Ok((book.get_recipe(i)?, result.value(variable))))
        .collect::<Result<Vec<(&Recipe, f64)>>>()?;


    Ok(ProblemOutput { book, used_recipes })
}
