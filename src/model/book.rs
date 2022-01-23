use std::collections::HashSet;
use crate::error::Result;
use crate::model::filtered_book::FilteredBook;
use crate::model::item::Item;
use crate::model::recipe::Recipe;
use crate::recipe_filter::RecipeFilter;

pub trait Book {

    /// The number of recipes this book contains
    fn number_of_recipes(&self) -> usize;

    /// Retrieve a recipe from its index in this book
    fn get_recipe(&self, recipe_index: usize) -> Result<&Recipe>;

    /// List all the items involved in all the recipes of this book
    fn get_involved_items(&self) -> Result<HashSet<Item>>;

    /// Retrieve an item based on its id
    fn get_item_by_id(&self, item_id:&str) -> Result<&Item>;

}

pub trait FilterableBook {

    fn filter(&self, predicate:&RecipeFilter) -> Result<FilteredBook>;

}