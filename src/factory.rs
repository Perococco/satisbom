use std::collections::HashMap;
use std::ops::{Mul};
use good_lp::{Constraint, Expression, Variable};
use crate::book::Book;
use crate::error::Result;
use crate::input::item::Item;
use crate::input::reactant::Reactant;
use crate::input::recipe::Recipe;
use crate::problem_input::ProblemInput;

pub struct Factory<'a> {
    book:&'a dyn Book,
    recipe_amounts:&'a[Variable],
    available_items:&'a HashMap<String,u32>,
    production:Production<'a>
}

#[derive(Default)]
pub struct Production<'a> {
    resource_quantities:HashMap<&'a str, Expression>,
    product_quantities:HashMap<&'a str, Expression>
}

impl<'a> Production<'a> {
    pub(crate) fn compute_constraints(self, input: &ProblemInput) -> Vec<Constraint> {

        let mut result = vec![];

        for (item_id, produced_quantity) in self.product_quantities.into_iter() {
            let constraint = match input.get_requested_quantity(item_id) {
                Some(quantity) => produced_quantity.eq(quantity),
                None => produced_quantity.geq(0)
            };

            result.push(constraint);

        }
        result
    }
}

impl<'a> Production<'a> {
    pub fn resource_quantities(&self) -> &HashMap<&'a str, Expression> {
        &self.resource_quantities
    }
}


impl Factory<'_> {
    pub fn compute_production<'a>(book: &'a dyn Book, recipe_amounts:&'a [Variable], available_items:&'a HashMap<String,u32>) -> Result<Production<'a>> {
        let mut factory = Factory { book, recipe_amounts, production:Default::default(), available_items};

        factory.compute()?;



        Ok(factory.production)
    }
}


impl <'a> Factory<'a> {

    fn compute(&mut self) -> Result<()> {

        for recipe_index in 0..self.book.number_of_recipes() {
            self.add_quantity_for_one_recipe(recipe_index)?
        }

        for (id,available) in self.available_items {
            let item = self.book.get_item_by_id(id)?;
            let quantities = match item {
                Item::Resource(_) => &mut self.production.resource_quantities,
                Item::Product(_) => &mut self.production.product_quantities
            };

            let entry = quantities.entry(id).or_default();
            *entry += *available as f64;
        }


        Ok(())
    }

    fn add_quantity_for_one_recipe(&mut self, recipe_index:usize) -> Result<()> {
        self.update_quantity_for_reactants(recipe_index, |r| r.inputs(), -1)?;
        self.update_quantity_for_reactants(recipe_index, |r| r.outputs(), 1)
    }

    fn update_quantity_for_reactants(&mut self, recipe_index:usize, getter:fn (&Recipe) -> &Vec<Reactant>, factor:i32) -> Result<()> {
        let recipe = self.book.get_recipe(recipe_index)?;
        let amount = &self.recipe_amounts[recipe_index];

        for reactant in getter(recipe) {

            let use_quantity = amount.mul((reactant.quantity() as i32) * factor);

            let item = self.book.get_item_by_id(reactant.item_id())?;
            let quantities = match item {
                Item::Resource(_) => &mut self.production.resource_quantities,
                Item::Product(_) => &mut self.production.product_quantities
            };

            let entry = quantities.entry(reactant.item_id()).or_default();
            *entry += use_quantity;
        };

        Ok(())
    }

}