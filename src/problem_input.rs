use std::collections::HashMap;

pub struct ProblemInput {
    pub requested_output:HashMap<String,u32>,
    pub available_items:HashMap<String,u32>,
    // pub _no_alternative:bool,
    // pub _use_abundance:bool,
}

impl ProblemInput {
    pub(crate) fn is_available_item(&self, item_id: &str) -> bool {
        self.available_items.contains_key(item_id)
    }
}

impl ProblemInput {

    pub fn available_items(&self) -> &HashMap<String,u32> {
        &self.available_items
    }

    pub fn get_requested_quantity(&self, item_id:&str) -> Option<u32> {
        self.requested_output.get(item_id).cloned()
    }

    pub fn is_requested_item(&self, item_id:&str) -> bool {
        self.requested_output.contains_key(item_id)
    }
}

