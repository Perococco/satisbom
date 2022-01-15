use std::collections::HashMap;
use std::ops::{Add, Sub};
use good_lp::{Constraint, Expression, IntoAffineExpression};
use crate::model::book::Book;
use crate::model::item::Item;
use crate::ProblemInput;

pub struct Production<'a> {
    _book: &'a dyn Book,
    input:&'a ProblemInput,
    resources: HashMap<Item, Expression>,
    leftovers: HashMap<Item, Expression>,
    targets: HashMap<Item, Expression>,
    available: HashMap<Item, Expression>,
}

impl<'a> Production<'a> {

    pub(crate) fn new(book:&'a dyn Book, input:&'a ProblemInput) -> Self {
        Production{_book:book,input,
            resources:Default::default(),
            leftovers:Default::default(),
            targets:Default::default(),
            available:Default::default()}
    }

    pub fn objective(&self) -> Expression {
        let sum_of_resources:Expression = self.resources.values().sum();

        let sum_of_available_items:Expression = self.available.values().sum();
        sum_of_resources.add(sum_of_available_items)
    }

    pub fn compute_constraints(&self) -> Vec<Constraint> {
        let mut result = vec![];

        for (item, produced_quantity) in &self.targets {
            let expression = Expression::from_other_affine(produced_quantity);
            let quantity = self.input.get_requested_quantity(item).unwrap();
            result.push(expression.eq(quantity as f64));
        }

        for (_,produced_quantity) in &self.leftovers {
            let expression = Expression::from_other_affine(produced_quantity);
            result.push(expression.geq(0));
        }
        
        for (_,produced_quantity) in &self.available {
            let expression = Expression::from_other_affine(produced_quantity);
            result.push(expression.geq(0));
        }

        result
    }

    pub fn add<RHS>(&mut self, item:& Item, value:RHS) where RHS: IntoAffineExpression {
        let requested_item = self.input.is_requested_item(item);
        let available_item = self.input.is_available_item(item);
        let quantities = match (item, requested_item, available_item) {
            (Item::Resource(_),_,_) => &mut self.resources,
            (Item::Product(_),false, false) => &mut self.leftovers,
            (Item::Product(_),false, true) => &mut self.available,
            (Item::Product(_),true, _) => &mut self.targets,
        };

        match quantities.get_mut(item) {
            None => {quantities.insert(item.clone(), Expression::from_other_affine(value));},
            Some(exp) => *exp+=value
        };

    }
}

