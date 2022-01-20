use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use hashlink::LinkedHashMap;
use crate::AmountFormat;
use crate::model::bom_printer::BomPrinter;
use crate::model::building::Building;
use crate::model::item::Item;
use crate::model::recipe::Recipe;
use crate::model::recipe_complexity::compute_complexity;

pub struct Bom {
    pub targets: HashMap<Item, f64>,
    pub requirements: HashMap<Item, f64>,
    pub leftovers: HashMap<Item, f64>,
    pub recipes: LinkedHashMap<Recipe, f64>,
    pub buildings: HashMap<Building, u32>,
}

impl Bom {
    pub fn new(targets: HashMap<Item, f64>, requirements: HashMap<Item, f64>, leftovers: HashMap<Item, f64>, recipes: HashMap<Recipe, f64>) -> Self {
        let mut buildings = HashMap::new();

        for (recipe, amount) in &recipes {
            let building = recipe.building();
            let q = (*amount / recipe.nb_per_minute()).ceil() as u32;
            match buildings.get_mut(building) {
                None => { buildings.insert(building.clone(), q); }
                Some(a) => *a += q
            }
        }


        let recipes = sort_recipes(recipes);

        Bom { targets, requirements, leftovers, recipes, buildings }
    }
}

fn sort_recipes(recipes: HashMap<Recipe, f64>) -> LinkedHashMap<Recipe,f64> {
    let mut recipes_vec:Vec<Recipe> = recipes.keys().cloned().collect();
    let complexity = compute_complexity(&recipes_vec);

    recipes_vec.sort_by(|r1,r2| complexity.get(r1.id()).cmp(&complexity.get(r2.id())));

    let mut r = LinkedHashMap::new();

    for recipe in recipes_vec {
        if let Some(v) = recipes.get(&recipe) {
            r.insert(recipe,*v);
        }
    }

    r
}


impl Display for Bom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = BomPrinter::with_formatter(f,AmountFormat::F64);
        self.display(&mut printer).map_err(|_| Error)
    }
}


impl Bom {
    pub fn display(&self, bp: &mut BomPrinter) -> crate::error::Result<()> {

        bp.display_items("To get:", &self.targets)?;
        bp.display_items("You need:", &self.requirements)?;
        bp.display_items("Leftovers:", &self.leftovers)?;

        bp.display_recipes(&self.recipes)?;

        bp.display_buildings(&self.buildings)
    }
}