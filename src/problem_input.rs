use std::collections::HashMap;

pub struct ProblemInput {
    pub requested_output:HashMap<String,u32>,
    pub available_items:HashMap<String,u32>,
    // pub _no_alternative:bool,
    // pub _use_abundance:bool,
}

impl ProblemInput {

    pub fn available_items(&self) -> &HashMap<String,u32> {
        &self.available_items
    }

    pub fn get_requested_quantity(&self, item_id:&str) -> Option<u32> {
        self.requested_output.get(item_id).cloned()
    }
}

