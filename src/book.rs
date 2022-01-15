use std::collections::HashSet;
use crate::error::Result;
use crate::model::filtered_book::FilteredBook;
use crate::model::item::Item;
use crate::model::recipe::Recipe;

pub trait Book {

    fn get_recipe(&self, recipe_index: usize) -> Result<&Recipe>;

    fn number_of_recipes(&self) -> usize;

    fn get_involved_items(&self) -> Result<HashSet<Item>>;

    fn get_item_by_id(&self, item_id:&str) -> Result<&Item>;

}

pub trait FilterableBook {

    fn filter(&self, predicate:& impl Fn(&Recipe) -> bool) -> Result<FilteredBook>;

}