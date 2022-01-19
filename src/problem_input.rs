use std::collections::HashMap;
use crate::model::item::Item;

pub struct ProblemInput {
    pub requested_output:HashMap<String,u32>,
    pub available_items:HashMap<String,u32>,
}

impl ProblemInput {

    pub fn available_items(&self) -> &HashMap<String,u32> {
        &self.available_items
    }

    pub fn get_requested_quantity(&self, item:&Item) -> Option<u32> {
        self.requested_output.get(item.id()).cloned()
    }

    pub fn is_requested_item(&self, item:&Item) -> bool {
        self.requested_output.contains_key(item.id())
    }
}

