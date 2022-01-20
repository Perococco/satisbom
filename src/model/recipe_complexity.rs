use std::collections::{HashMap, HashSet};
use crate::Recipe;




struct Helper<'a> {
    recipes: &'a [Recipe],
    recipe_by_output_item_id: HashMap<String, Vec<usize>>,
    items: HashSet<String>,
}

impl<'a> Helper<'a> {
    pub fn new(recipes: &'a [Recipe]) -> Self {
        let mut o = HashMap::new();
        let mut items = HashSet::new();

        for (idx, recipe) in recipes.iter().enumerate() {
            for input in recipe.inputs() {
                let item_id = input.item_id();
                items.insert(item_id.to_string());
            }

            for output in recipe.outputs() {
                let item_id = output.item_id();
                items.insert(item_id.to_string());
                match o.get_mut(item_id) {
                    None => {o.insert(item_id.to_string(), vec![idx]);},
                    Some(v) => {v.push(idx);}
                }
            }
        }

        Helper { recipes, recipe_by_output_item_id: o, items }
    }
}


struct ItemComplexity {
    complexities: HashMap<String, u32>,
}

impl ItemComplexity {
    pub fn compute_recipe_complexity(&self, recipe:&Recipe) -> u32 {
        recipe.inputs().iter().map(|r| self.complexities.get(r.item_id()))
            .flatten()
            .max().cloned()
            .unwrap_or(0)
    }
}

impl ItemComplexity {
    pub fn compute(helper: &Helper) -> Self {

        let mut complexities = HashMap::new();
        let mut visited = HashSet::new();

        for item in &helper.items {
            compute_item_complexity(item,&helper,&mut complexities,&mut visited);
        };

        ItemComplexity{complexities}
    }
}


fn compute_item_complexity(item: &str, helper: &Helper<'_>, complexities: &mut HashMap<String, u32>, visited: &mut HashSet<String>) -> u32 {
    if let Some(c) = complexities.get(item) {
        return *c;
    }

    visited.insert(item.to_string());
    let recipe_indices = helper.recipe_by_output_item_id.get(item);

    let complexity = if let Some(v) = recipe_indices {
        let inputs: HashSet<String> = v.iter()
            .map(|i| &helper.recipes[*i])
            .flat_map(|r| r.inputs().iter())
            .map(|r| r.item().id())
            .map(|i| i.to_string())
            .collect();

        let mut complexity = 0;
        for input in inputs {
            let item_complexity = {
                if visited.contains(&input) {
                    complexities.get(&input).cloned().unwrap_or(0)
                } else {
                    compute_item_complexity(&input, helper, complexities, visited)
                }
            };
            complexity = complexity.max(item_complexity+1);
        }
        complexity
    } else {
        0
    };

    complexities.insert(item.to_string(),complexity);
    complexity
}

// impl Helper<'_> {
//     pub(crate) fn set_complexity(&mut self, recipe_index: usize, complexity: u32) {
//         self.complexities.insert(recipe_index, complexity);
//     }
// }
//
//
// impl<'a> Helper<'a> {
//     fn new(recipes: &'a [Recipe]) -> Self {
//         let mut recipe_by_output_item_id: HashMap<String, Vec<usize>> = Default::default();
//
//         for (i, recipe) in recipes.iter().enumerate() {
//             for reactant in recipe.outputs() {
//                 let item_id = reactant.item_id();
//                 match recipe_by_output_item_id.get_mut(item_id) {
//                     Some(v) => v.push(i),
//                     None => {
//                         recipe_by_output_item_id.insert(item_id.to_string(), vec![i]);
//                     }
//                 }
//             }
//         }
//
//         Helper { recipes, complexities: Default::default(), recipe_by_output_item_id, in_progress: Default::default() }
//     }
//
//     fn complexity_of(&self, recipe_index: usize) -> Option<u32> {
//         if self.in_progress.contains(&recipe_index) {
//             Some(0)
//         } else {
//             self.complexities.get(&recipe_index).cloned()
//         }
//     }
//
//     fn recipes_producing(&self, item: &Item) -> Option<Vec<usize>> {
//         self.recipe_by_output_item_id.get(item.id()).cloned()
//     }
//
//     fn complexity_to_produce(&mut self, item: &Item) -> u32 {
//         match item {
//             Item::Resource(_) => 0,
//             Item::Product(_) => if let Some(recipe_indices) = self.recipes_producing(item) {
//                 let mut c = u32::MAX;
//                 for recipe_index in recipe_indices {
//                     let item_complexity = 1 + self.compute_complexity_rec(recipe_index);
//                     c = item_complexity.min(c)
//                 }
//                 c
//             } else {
//                 0
//             },
//         }
//     }
//
//     fn set_in_progress(&mut self, recipe_index: usize) {
//         self.in_progress.insert(recipe_index);
//     }
//
//     fn unset_in_progress(&mut self, recipe_index: &usize) {
//         self.in_progress.remove(recipe_index);
//     }
//
//
//     fn compute_complexity_rec(&mut self, recipe_index: usize) -> u32 {
//         if let Some(c) = self.complexity_of(recipe_index) {
//             return c;
//         }
//         self.set_in_progress(recipe_index);
//
//         let recipe = &self.recipes[recipe_index];
//
//         let c = recipe.inputs()
//             .iter()
//             .map(|r| self.complexity_to_produce(r.item()))
//             .max()
//             .unwrap_or(0);
//
//         self.unset_in_progress(&recipe_index);
//
//         self.set_complexity(recipe_index, c);
//         c
//     }
// }
//
//
pub fn compute_complexity(recipes: &[Recipe]) -> HashMap<String, u32> {
    let helper = Helper::new(recipes);
    let item_complexity = ItemComplexity::compute(&helper);


    let mut recipe_complexity = HashMap::new();

    for recipe in recipes {
        let c = item_complexity.compute_recipe_complexity(recipe);
        recipe_complexity.insert(recipe.id().to_string(),c);
    };

    recipe_complexity
}
//
//
