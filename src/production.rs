use std::collections::HashMap;
use std::ops::Add;

use good_lp::{Constraint, Expression, IntoAffineExpression};
use good_lp::solvers::lp_solvers::LpSolution;
use crate::bag::{Bag, HashBag};

use crate::model::book::Book;
use crate::model::item::Item;
use crate::ProblemInput;

pub struct Production<T> {
    requested_output: HashBag<Item, f64>,
    available_items: HashBag<Item, f64>,
    resources: HashBag<Item, T>,
    leftovers: HashBag<Item, T>,
    targets: HashBag<Item, T>,
    available_left: HashBag<Item, T>,
}

impl<T> Production<T> {
    pub fn requested_output(&self) -> &HashBag<Item, f64> {
        &self.requested_output
    }
    pub fn available_items(&self) -> &HashBag<Item, f64> {
        &self.available_items
    }
    pub fn resources(&self) -> &HashBag<Item, T> {
        &self.resources
    }
    pub fn leftovers(&self) -> &HashBag<Item, T> {
        &self.leftovers
    }
    pub fn targets(&self) -> &HashBag<Item, T> {
        &self.targets
    }
    pub fn available_left(&self) -> &HashBag<Item, T> {
        &self.available_left
    }
}


impl Production<Expression> {

    pub(crate) fn new(book:&dyn Book, input:&ProblemInput) -> crate::error::Result<Self> {
        let requested_output = convert_input(&input.requested_output,book)?;
        let available_items = convert_input(&input.available_items,book)?;
        Ok(Production {
            requested_output,
            available_items,
            resources: Default::default(),
            leftovers: Default::default(),
            targets: Default::default(),
            available_left: Default::default(),
        })
    }

    pub fn objective(&self) -> Expression {
        let sum_of_resources:Expression = self.resources.values().sum();

        let sum_of_available_items:Expression = self.available_left.values().sum();
        sum_of_resources.add(sum_of_available_items)
    }

    pub fn compute_constraints(&self) -> Vec<Constraint> {
        let mut result = vec![];

        for (item, produced_quantity) in self.targets.iter() {
            let expression = Expression::from_other_affine(produced_quantity);
            let quantity = self.requested_output
                .get(item)
                .cloned()
                .unwrap_or(0f64);
            result.push(expression.geq(quantity as f64));
        }

        for produced_quantity in self.leftovers.values() {
            let expression = Expression::from_other_affine(produced_quantity);
            result.push(expression.geq(0));
        }
        
        for produced_quantity in self.available_left.values() {
            let expression = Expression::from_other_affine(produced_quantity);
            result.push(expression.geq(0));
        }

        result
    }

    pub fn add<RHS>(&mut self, item:& Item, value:RHS) where RHS: IntoAffineExpression {
        let requested_item = self.requested_output.contains_key(item);
        let available_item = self.available_items.contains_key(item);
        let quantities = match (item, requested_item, available_item) {
            (Item::Resource(_),_,_) => &mut self.resources,
            (Item::Product(_),false, false) => &mut self.leftovers,
            (Item::Product(_),false, true) => &mut self.available_left,
            (Item::Product(_),true, _) => &mut self.targets,
        };

        quantities.add_item(item.clone(), Expression::from_other_affine(value));

    }


}

fn convert_input(input:&HashMap<String,u32>, book:&dyn Book) -> crate::error::Result<HashBag<Item,f64>>{
        input.iter()
            .map(|(item_id,amount)| Ok((book.get_item_by_id(item_id)?.clone(),*amount as f64)))
            .collect()
}

impl Production<Expression>  {

    pub fn evaluate(self, solution:&LpSolution) -> Production<f64> {
        let targets = evaluate(&self.targets, solution,1f64);
        let resources = evaluate(&self.resources, solution, -1f64);
        let leftovers = evaluate(&self.leftovers, solution,1f64);
        let available = evaluate(&self.available_left, solution, 1f64);


        Production{
            available_items:self.available_items,
            requested_output:self.requested_output,
            resources,leftovers,targets, available_left: available }
    }
}

fn evaluate(items:&HashBag<Item,Expression>, result:&LpSolution, factor:f64) -> HashBag<Item,f64> {
    let mut bag:HashBag<Item,f64> = items.iter().map(|(item, e)| (item.clone(), e.eval_with(result) * factor)).collect();
    bag.clean();
    bag
}
