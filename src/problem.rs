use std::collections::HashMap;
use std::ops::{Div, Mul};

use good_lp::{Constraint, default_solver, Expression, IntoAffineExpression, ProblemVariables, Solution, SolverModel, Variable};
use good_lp::solvers::lp_solvers::LpSolution;
use hashlink::LinkedHashMap;

use crate::model::item::{Item};
use crate::{Bom, Recipe};
use crate::constants::is_nil;

pub struct Problem {
    variables: ProblemVariables,
    data: ProblemData,
}

struct ProblemData {
    target_items: HashMap<Item, f64>,
    available_items: HashMap<Item, f64>,
    use_abundances: bool,

    recipe_amount: LinkedHashMap<Recipe, Variable>,
    item_count: HashMap<Item, Expression>,

}

impl Problem {
    pub fn new(variables: ProblemVariables,
               target_items: HashMap<Item, f64>,
               available_items: HashMap<Item, f64>,
               recipe_amount: LinkedHashMap<Recipe, Variable>,
               item_count: HashMap<Item, Expression>,
               use_abundances: bool) -> Self {
        Problem { variables, data: ProblemData { target_items, available_items, use_abundances, recipe_amount, item_count } }
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

        let total: u32 = self.item_count.keys()
            .map(|i| i.as_resource())
            .flatten()
            .map(|r| r.max_quantity_per_minute())
            .flatten()
            .sum();

        for (item, e) in &self.item_count {
            if self.target_items.contains_key(item) {
                continue;
            }

            let available_items = self.available_items.contains_key(item);
            let target = self.target_items.contains_key(item);

            let e = e.clone().mul(total);
            match (item, target, available_items) {
                (Item::Resource(r), _, _) => {
                    if let Some(mq) = r.max_quantity_per_minute() {
                        objective -= e.div(if self.use_abundances { mq } else { 1 });
                    } else {
                        objective -= e.div(1000000000)
                    }
                }
                _ => {}
            }
        }

        for (_, amount) in &self.recipe_amount {
            objective+=amount
        }
        objective
    }

    fn compute_constraints(&self) -> Vec<Constraint> {
        let mut constraints = vec![];
        for (item, e) in &self.item_count {
            let target = self.target_items.get(item).cloned();
            let available = self.available_items.get(item).cloned();
            match item {
                Item::Resource(r) => {
                    constraints.push(e.clone().leq(0));
                    if let Some(q) = r.max_quantity_per_minute() {
                        constraints.push(e.clone().geq(-(q as f64)));
                    }
                }
                Item::Product(_) => {
                    let constraint = match (target, available) {
                        (Some(amount), _) => e.clone().eq(amount),
                        _ => e.clone().geq(0)
                    };
                    constraints.push(constraint);
                }
            }
        }
        constraints
    }
}


impl ProblemData {
    fn create_boom(self, solution: LpSolution) -> Bom {
        let recipes = self.recipe_amount.into_iter()
            .map(|(recipe, variable)| (recipe, solution.value(variable)))
            .filter(|(_, a)| !is_nil(*a))
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


        Bom::new(targets, self.available_items, requirements, leftovers, recipes)
    }
}
