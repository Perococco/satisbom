use std::collections::HashMap;
use crate::NotManual;
use crate::recipe_filter::RecipeFilter;

#[derive(serde::Deserialize,serde::Serialize, Debug)]
pub struct ProblemInput {
    #[serde(rename="targets")]
    pub target_items:HashMap<String,u32>,
    #[serde(rename="available-items")]
    pub available_items:HashMap<String,u32>,
    #[serde(rename="use-abundances")]
    pub use_abundances:bool,
    pub filter:RecipeFilter
}


impl Default for ProblemInput {
    fn default() -> Self {
        ProblemInput{
            target_items:HashMap::new(),
            available_items:HashMap::new(),
            use_abundances:true,
            filter:NotManual,
        }
    }
}

impl ProblemInput {

    pub fn target_items(&self) -> &HashMap<String, u32> {
        &self.target_items
    }

    pub fn available_items(&self) -> &HashMap<String,u32> {
        &self.available_items
    }

    pub fn filter(&self) -> &RecipeFilter {
        &self.filter
    }

}

