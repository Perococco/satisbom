use std::collections::{HashMap, HashSet};
use crate::model::book::{Book, FilterableBook};
use crate::error::{Error, Result};
use crate::error::Error::{InvalidRecipeIndex};
use crate::model::dto::book::BookDto;
use crate::model::filtered_book::FilteredBook;
use crate::model::item::Item;
use crate::model::recipe::Recipe;


pub struct FullBook {
    items:HashMap<String,Item>,
    recipes:Vec<Recipe>
}


impl FullBook {

    pub fn create() -> Result<Self> {
        let dto = BookDto::parse()?;
        dto.to_full_book()
    }


    pub fn new(items: HashMap<String, Item>, recipes: Vec<Recipe>) -> Self {
        FullBook { items, recipes }
    }
}

impl FilterableBook for FullBook {
    fn filter(&self, predicate: &impl Fn(&Recipe) -> bool) -> Result<FilteredBook> {
        let filtered_recipes = self.recipes
            .iter()
            .enumerate()
            .filter(|(_, r)| predicate(r))
            .map(|(i, _)| i)
            .collect();
        Ok(FilteredBook::new(self, filtered_recipes))
    }
}

impl Book for FullBook {

    fn get_recipe(&self, recipe_index: usize) -> Result<&Recipe> {
        self.recipes.get(recipe_index).ok_or(InvalidRecipeIndex(recipe_index))
    }

    fn number_of_recipes(&self) -> usize {
        self.recipes.len()
    }

    fn get_involved_items(&self) -> crate::error::Result<HashSet<Item>> {
        Ok(self.recipes
            .iter()
            .flat_map(|r| r.get_involved_items())
            .cloned()
            .collect())
    }

    fn get_item_by_id(&self, item_id:&str) -> Result<&Item> {
        self.items.get(item_id).ok_or_else(|| Error::UnknownItem(item_id.to_string()))
    }
}


