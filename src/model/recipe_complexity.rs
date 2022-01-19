use std::collections::{HashMap, HashSet};
use crate::model::item::Item;
use crate::Recipe;

struct Helper<'a> {
    recipes: &'a [Recipe],
    complexities: HashMap<usize, u32>,
    recipe_by_output_item_id: HashMap<String, Vec<usize>>,
    in_progress: HashSet<usize>,
}

impl Helper<'_> {
    pub(crate) fn set_complexity(&mut self, recipe_index: usize, complexity: u32) {
        self.complexities.insert(recipe_index, complexity);
    }
}


impl<'a> Helper<'a> {

    fn new(recipes: &'a [Recipe]) -> Self {
        let mut recipe_by_output_item_id: HashMap<String, Vec<usize>> = Default::default();

        for (i, recipe) in recipes.iter().enumerate() {
            for reactant in recipe.outputs() {
                let item_id = reactant.item_id();
                match recipe_by_output_item_id.get_mut(item_id) {
                    Some(v) => v.push(i),
                    None => {
                        recipe_by_output_item_id.insert(item_id.to_string(), vec![i]);
                    }
                }
            }
        }

        Helper { recipes, complexities: Default::default(), recipe_by_output_item_id, in_progress: Default::default() }
    }

    fn complexity_of(&self, recipe_index: usize) -> Option<u32> {
        if self.in_progress.contains(&recipe_index) {
            Some(0)
        } else {
            self.complexities.get(&recipe_index).cloned()
        }
    }

    fn recipes_producing(&self, item: &Item) -> Option<Vec<usize>> {
        self.recipe_by_output_item_id.get(item.id()).cloned()
    }

    fn complexity_to_produce(&mut self, item: &Item) -> u32 {
        match item {
            Item::Resource(_) => 0,
            Item::Product(_) => if let Some(recipe_indices) = self.recipes_producing(item) {
                let mut c = u32::MAX;
                for recipe_index in recipe_indices {
                    let item_complexity = 1+self.compute_complexity_rec(recipe_index);
                    c = item_complexity.min(c)
                }
                c
            } else {
                0
            },
        }
    }

    fn set_in_progress(&mut self, recipe_index: usize) {
        self.in_progress.insert(recipe_index);
    }

    fn unset_in_progress(&mut self, recipe_index: &usize) {
        self.in_progress.remove(recipe_index);
    }


    fn compute_complexity_rec(&mut self, recipe_index: usize) -> u32 {
        if let Some(c) = self.complexity_of(recipe_index) {
            return c;
        }
        self.set_in_progress(recipe_index);

        let recipe = &self.recipes[recipe_index];

        let c = recipe.inputs()
            .iter()
            .map(|r| self.complexity_to_produce(r.item()))
            .max()
            .unwrap_or(0);

        self.unset_in_progress(&recipe_index);

        self.set_complexity(recipe_index, c);
        c
    }
}


pub fn compute_complexity(recipes: &[Recipe]) -> HashMap<String, u32> {
    let mut helper = Helper::new(recipes);

    for recipe_index in 0..recipes.len() {
        helper.compute_complexity_rec(recipe_index);
    };

    helper.complexities
        .into_iter()
        .map(|(recipe_index, complexity)| (recipes[recipe_index].id().to_string(), complexity))
        .collect()
}


