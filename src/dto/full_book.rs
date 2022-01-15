use std::collections::{HashMap, HashSet};
use crate::dto::item::Item;
use crate::book::{Book, FilterableBook};
use crate::dto::filtered_book::FilteredBook;
use crate::dto::recipe::Recipe;
use crate::error::Result;
use crate::error::Error;
use crate::error::Error::{InvalidRecipeIndex};
use crate::ReferenceBook;


pub struct FullBook {
    reference_book:ReferenceBook,
    item_index_per_id:HashMap<String,usize>,

}


impl FullBook {

    pub fn create() -> Result<Self> {
        let reference_book = ReferenceBook::parse()?;
        let mut item_index_per_id = HashMap::new();
        for (index,item) in reference_book.items.iter().enumerate() {
            item_index_per_id.insert(item.get_id().to_string(),index);
        }

        Ok(FullBook{reference_book, item_index_per_id})
    }
}

impl FilterableBook for FullBook {
    fn filter(&self, predicate: &impl Fn(&Recipe) -> bool) -> Result<FilteredBook> {
        let filtered_recipes = self.reference_book.recipes
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
        self.reference_book.recipes.get(recipe_index).ok_or_else(|| InvalidRecipeIndex(recipe_index))
    }

    fn number_of_recipes(&self) -> usize {
        self.reference_book.recipes.len()
    }

    fn get_item_index(&self, item_id:&str) -> Result<usize> {
        self.item_index_per_id
            .get(item_id)
            .cloned()
            .ok_or_else(|| Error::UnknownItem(item_id.to_string()))
    }

    fn get_item_by_id(&self, item_id: &str) -> Result<&Item> {
        self.get_item_index(item_id).map(|i| &self.reference_book.items[i])
    }

    fn get_involved_item_indices(&self) -> Result<HashSet<usize>> {
        self.reference_book.recipes
            .iter().flat_map(|r| r.get_involved_item_indices(self))
            .collect()
    }
}


