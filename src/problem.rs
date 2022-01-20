use std::collections::HashMap;
use std::ops::Mul;

use good_lp::{Constraint, default_solver, Expression, IntoAffineExpression, ProblemVariables, Solution, SolverModel, Variable};
use good_lp::solvers::lp_solvers::LpSolution;
use hashlink::LinkedHashMap;

use crate::model::item::{Item};
use crate::{Bom, Recipe};
use crate::constants::is_nil;

pub struct Problem {
    variables: ProblemVariables,
    data:ProblemData
}

struct ProblemData {
    target_items: HashMap<Item, f64>,
    _available_items: HashMap<Item, f64>,

    recipe_amount: LinkedHashMap<Recipe, Variable>,
    item_count: HashMap<Item, Expression>,

}

impl Problem {
    pub fn new(variables: ProblemVariables, target_items: HashMap<Item, f64>, available_items: HashMap<Item, f64>, recipe_amount: LinkedHashMap<Recipe, Variable>, item_count: HashMap<Item, Expression>) -> Self {
        Problem { variables, data:ProblemData{target_items, _available_items:available_items, recipe_amount, item_count} }
    }
}


impl Problem {
    pub(crate) fn solve(self) -> crate::error::Result<Bom> {

        let data = self.data;

        let objective = data.objective();
        let variables = self.variables.minimise(objective).using(default_solver);
        let variables = data.compute_constraints().into_iter().fold(variables, |v, c| v.with(c));

        let result = variables.solve()?;

        Ok(data.create_boom(result))
    }
}
impl ProblemData {

    fn objective(&self) -> Expression {
        let mut objective = Expression::from(0);
        for (item, e) in &self.item_count {
            if self.target_items.contains_key(item) {
                continue;
            }

            let available = self._available_items.contains_key(item);

            match (item, available) {
                (Item::Resource(r),_) => {
                    if r.max_quantity().is_some() {
                        objective -= e;
                    } else {
                        objective -= e.clone().mul(1e-5)
                    }
                }
                (Item::Product(_),true) => { objective += e; }
                (_,_) => {}
            }
        }
        objective
    }

    fn compute_constraints(&self) -> Vec<Constraint> {
        let mut constraints = vec![];
        for (item, e) in &self.item_count {
            let target = self.target_items.get(item).cloned().unwrap_or(0f64);
            match item {
                Item::Resource(r) => {
                    constraints.push(e.clone().leq(0));
                    if let Some(q) = r.max_quantity() {
                        constraints.push(e.clone().geq(-(q as f64)));
                    }
                }
                Item::Product(_) => { constraints.push(e.clone().geq(target)) }
            }
        }
        constraints
    }
}


impl ProblemData {
    fn create_boom(self, solution: LpSolution) -> Bom {
        let recipes = self.recipe_amount.into_iter()
            .map(|(recipe, variable)| (recipe, solution.value(variable)))
            .filter(|(_,a)| !is_nil(*a))
            .collect();


        let mut targets = HashMap::new();
        let mut requirements = HashMap::new();
        let mut leftovers = HashMap::new();

        for (item, expression) in self.item_count {
            let amount = expression.eval_with(&solution);
            if is_nil(amount) {
                continue;
            }
            let target = self.target_items.contains_key(&item);
            match (&item, target) {
                (Item::Resource(_), _) => requirements.insert(item, -amount),
                (Item::Product(_), true) => targets.insert(item, amount),
                (Item::Product(_), false) => leftovers.insert(item, amount)
            };
        }


        Bom::new(targets, requirements, leftovers, recipes)
    }
}


//
// impl Problem<Variable,Expression> {
//
//     pub(crate) fn new(book:&dyn Book,
//                       input:&ProblemInput,
//         problem:&mut ProblemVariables
//     ) -> crate::error::Result<Self> {
//         let nb_recipes = book.number_of_recipes();
//         let recipes: Vec<Variable> = problem.add_vector(variable().min(0), nb_recipes);
//         let requested_output = convert_input(&input.requested_output,book)?;
//         let available_items = convert_input(&input.available_items,book)?;
//
//
//         Ok(Problem {
//             requested_output,
//             available_items,
//             resources: Default::default(),
//             leftovers: Default::default(),
//             targets: Default::default(),
//             available_left: Default::default(),
//         })
//     }
//
//     pub fn objective_to_minimize(&self) -> Expression {
//         let sum_of_resources:Expression = self.resources.values().sum();
// //             .iter()
// //             .map(|(i,e)| i.as_resource().map(|r| (r,e)))
// //             .flatten()
// // //            .filter(|(r,_)| r.max_quantity().is_some())
// //             .map(|(_,e)| e)
// //             .sum();
//
//         let sum_of_available_items:Expression = self.available_left.values().sum();
//         sum_of_resources.add(sum_of_available_items)
//     }
//
//     pub fn compute_constraints(&self) -> Vec<Constraint> {
//         let mut result = vec![];
//
//
//         // for (item,produced_quantity) in self.resources.iter() {
//         //     if let Item::Resource(r) = item {
//         //         if let Some(quantity) = r.max_quantity() {
//         //             let expression = Expression::from_other_affine(produced_quantity);
//         //             result.push(expression.leq(quantity))
//         //         }
//         //     }
//         // }
//
//         for (item, produced_quantity) in self.targets.iter() {
//             let expression = Expression::from_other_affine(produced_quantity);
//             let quantity = self.requested_output
//                 .get(item)
//                 .cloned()
//                 .unwrap_or(0f64);
//             result.push(expression.geq(quantity as f64));
//         }
//
//         for produced_quantity in self.leftovers.values() {
//             let expression = Expression::from_other_affine(produced_quantity);
//             result.push(expression.geq(0));
//         }
//
//         for produced_quantity in self.available_left.values() {
//             let expression = Expression::from_other_affine(produced_quantity);
//             result.push(expression.geq(0));
//         }
//
//         result
//     }
//
//     pub fn add<RHS>(&mut self, item:& Item, value:RHS) where RHS: IntoAffineExpression {
//         let requested_item = self.requested_output.contains_key(item);
//         let available_item = self.available_items.contains_key(item);
//         let quantities = match (item, requested_item, available_item) {
//             (Item::Resource(_),_,_) => &mut self.resources,
//             (Item::Product(_),false, false) => &mut self.leftovers,
//             (Item::Product(_),false, true) => &mut self.available_left,
//             (Item::Product(_),true, _) => &mut self.targets,
//         };
//
//         quantities.add_item(item.clone(), Expression::from_other_affine(value));
//
//     }
//
//
// }
//
// fn convert_input(input:&HashMap<String,u32>, book:&dyn Book) -> crate::error::Result<HashBag<Item,f64>>{
//         input.iter()
//             .map(|(item_id,amount)| Ok((book.get_item_by_id(item_id)?.clone(),*amount as f64)))
//             .collect()
// }
//
// impl Problem<Expression>  {
//
//     pub fn evaluate(self, solution:&LpSolution) -> Problem<f64> {
//         let targets = evaluate(&self.targets, solution,1f64);
//         let resources = evaluate(&self.resources, solution, -1f64);
//         let leftovers = evaluate(&self.leftovers, solution,1f64);
//         let available = evaluate(&self.available_left, solution, 1f64);
//
//
//         Problem {
//             available_items:self.available_items,
//             requested_output:self.requested_output,
//             resources,leftovers,targets, available_left: available }
//     }
// }
//
// fn evaluate(items:&HashBag<Item,Expression>, result:&LpSolution, factor:f64) -> HashBag<Item,f64> {
//     let mut bag:HashBag<Item,f64> = items.iter().map(|(item, e)| (item.clone(), e.eval_with(result) * factor)).collect();
//     bag.clean();
//     bag
// }
