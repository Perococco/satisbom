use std::collections::HashSet;
use crate::dto::filtered_book::FilteredBook;
use crate::dto::recipe::Recipe;
use crate::error::Result;
use crate::dto::item::Item;

pub trait Book {

    fn get_recipe(&self, recipe_index: usize) -> Result<&Recipe>;

    fn number_of_recipes(&self) -> usize;

    fn get_item_index(&self, item_id:&str) -> Result<usize>;

    fn get_item_by_id(&self, item_id:&str) -> Result<&Item>;

    fn get_involved_item_indices(&self) -> Result<HashSet<usize>>;


}

pub trait FilterableBook {

    fn filter(&self, predicate:& impl Fn(&Recipe) -> bool) -> Result<FilteredBook>;

}