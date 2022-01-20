use std::collections::HashMap;
use crate::model::item::Item;

pub struct ProblemInput {
    pub target_items:HashMap<String,u32>,
    pub available_items:HashMap<String,u32>,
}

impl ProblemInput {

    pub fn target_items(&self) -> &HashMap<String, u32> {
        &self.target_items
    }

    pub fn available_items(&self) -> &HashMap<String,u32> {
        &self.available_items
    }

    pub fn get_requested_quantity(&self, item:&Item) -> Option<u32> {
        self.target_items.get(item.id()).cloned()
    }

    pub fn is_requested_item(&self, item:&Item) -> bool {
        self.target_items.contains_key(item.id())
    }


}

