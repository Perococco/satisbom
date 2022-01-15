use std::ops::{Mul};
use good_lp::Variable;
use crate::book::Book;
use crate::error::Result;
use crate::model::reactant::Reactant;
use crate::model::recipe::Recipe;
use crate::problem_input::ProblemInput;
use crate::production::Production;

pub struct Factory<'a> {
    book: &'a dyn Book,
    input:&'a ProblemInput,
    recipe_amounts: &'a [Variable],
    production: Production<'a>,
}


impl Factory<'_> {
    pub fn compute_production<'a>(book: &'a dyn Book,
                                  input:&'a ProblemInput,
                                  recipe_amounts: &'a [Variable]) -> Result<Production<'a>> {
        let mut factory = Factory { book:book, input, recipe_amounts, production: Production::new(book,input) };

        factory.compute()?;


        Ok(factory.production)
    }
}


impl<'a> Factory<'a> {
    fn compute(&mut self) -> Result<()> {

        for recipe_index in 0..self.book.number_of_recipes() {
            self.add_quantity_for_one_recipe(recipe_index)?
        }

        for (item_id, available) in &self.input.available_items {
            let item = self.book.get_item_by_id(item_id)?;
            self.production.add(item,*available);
        }

        Ok(())
    }

    fn add_quantity_for_one_recipe(&mut self, recipe_index: usize) -> Result<()> {
        self.update_quantity_for_reactants(recipe_index, |r| r.inputs(), -1)?;
        self.update_quantity_for_reactants(recipe_index, |r| r.outputs(), 1)
    }

    fn update_quantity_for_reactants(&mut self, recipe_index: usize, getter: fn(&Recipe) -> &[Reactant], factor: i32) -> Result<()> {
        let recipe = self.book.get_recipe(recipe_index)?;
        let amount = &self.recipe_amounts[recipe_index];

        for reactant in getter(recipe) {
            let item = reactant.item();

            let use_quantity = amount.mul((reactant.quantity() as i32) * factor);
            self.production.add(item,use_quantity);
        };

        Ok(())
    }
}