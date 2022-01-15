use std::collections::HashMap;
use std::ops::Sub;
use good_lp::{Constraint, Expression, IntoAffineExpression};
use crate::book::Book;
use crate::dto::item::Item;
use crate::ProblemInput;

pub struct Production<'a> {
    _book: &'a dyn Book,
    input:&'a ProblemInput,
    resources: HashMap<&'a str, Expression>,
    leftovers: HashMap<&'a str, Expression>,
    targets: HashMap<&'a str, Expression>,
}

impl<'a> Production<'a> {

    pub(crate) fn new(book:&'a dyn Book, input:&'a ProblemInput) -> Self {
        Production{_book:book,input,resources:Default::default(),leftovers:Default::default(),targets:Default::default(),}
    }

    pub fn objective(&self) -> Expression {
        let sum_of_resources:Expression = self.resources.values().sum();
        let sum_of_left_overs:Expression = self.leftovers.iter()
            .filter(|(id,_)| !self.input.is_available_item(id))
            .map(|(_,e)| e)
            .sum();
        sum_of_resources.sub(sum_of_left_overs)
    }

    pub fn compute_constraints(&self) -> Vec<Constraint> {
        let mut result = vec![];

        for (item_id, produced_quantity) in &self.targets {
            let expression = Expression::from_other_affine(produced_quantity);
            let quantity = self.input.get_requested_quantity(item_id).unwrap();
            result.push(expression.eq(quantity as f64));
        }

        for (_,produced_quantity) in &self.leftovers {
            let expression = Expression::from_other_affine(produced_quantity);
            result.push(expression.geq(0));
        }
        
        result
    }

    pub fn add<RHS>(&mut self, item:&'a Item, value:RHS) where RHS: IntoAffineExpression {
        let requested_item = self.input.is_requested_item(item.get_id());
        let quantities = match (item,requested_item) {
            (Item::Resource(_),_) => &mut self.resources,
            (Item::Product(_),false) => &mut self.leftovers,
            (Item::Product(_),true) => &mut self.targets,
        };

        let entry = quantities.entry(item.get_id()).or_default();
        *entry += value;
    }
}

