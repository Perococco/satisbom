use std::collections::HashMap;
use std::ops::{Mul, Neg};
use good_lp::{Expression, ProblemVariables, Variable, variable};
use hashlink::LinkedHashMap;
use crate::model::book::Book;
use crate::error::Result;
use crate::model::item::Item;
use crate::model::recipe::Recipe;
use crate::problem_input::ProblemInput;
use crate::problem::Problem;

pub struct Factory<'b> {
    book: &'b dyn Book,
    input: &'b ProblemInput,
    variables: ProblemVariables,
    recipes: LinkedHashMap<Recipe, Variable>,
}


impl Factory<'_> {
    pub fn create_problem<'b>(input: &'b ProblemInput, book: &'b dyn Book) -> Result<Problem> {
        let mut variables = ProblemVariables::new();

        let mut recipes = LinkedHashMap::new();
        let nb_recipes = book.number_of_recipes();
        for recipe_index in 0..nb_recipes {
            let recipe = book.get_recipe(recipe_index)?;
            let definition = variables.add(variable().min(0));
            recipes.insert(recipe.clone(), definition);
        }

        Factory {
            book,
            input,
            variables,
            recipes,
        }.create()
    }
}


impl<'b> Factory<'b> {
    fn create(self) -> Result<Problem> {
        let mut items = HashMap::new();

        let target_items = convert_map(self.input.target_items(), self.book)?;
        let available_items = convert_map(self.input.available_items(), self.book)?;

        for (recipe, variable) in &(self.recipes) {
            for input in recipe.inputs() {
                let quantity = variable.mul(input.quantity());
                remove_item_quantity(&mut items, input.item(), quantity)
            }
            for output in recipe.outputs() {
                let quantity = variable.mul(output.quantity());
                add_item_quantity(&mut items, output.item(), quantity)
            }
        }

        for (item, count) in &available_items {
            add_item_quantity(&mut items, item, Expression::from(*count))
        }


        Ok(Problem::new(self.variables, target_items, available_items, self.recipes, items))
    }
}

fn add_item_quantity(items: &mut HashMap<Item, Expression>, item: &Item, value: Expression) {
    match items.get_mut(item) {
        Some(e) => { *e += value; }
        None => { items.insert(item.clone(), value); }
    }
}

fn remove_item_quantity(items: &mut HashMap<Item, Expression>, item: &Item, value: Expression) {
    match items.get_mut(item) {
        Some(e) => { *e -= value; }
        None => { items.insert(item.clone(), value.neg()); }
    }
}


fn convert_map(items: &HashMap<String, u32>, book: &dyn Book) -> Result<HashMap<Item, f64>> {
    let mut result = HashMap::new();
    for (item_id, quantity) in items {
        let item = book.get_item_by_id(item_id)?.clone();
        result.insert(item, *quantity as f64);
    };

    Ok(result)
}
