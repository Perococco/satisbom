use std::collections::HashMap;
use crate::model::building::Building;
use crate::model::item::Item;

use crate::error::{Error, Result};
use crate::model::dto::book::BookDto;
use crate::model::dto::reactant::ReactantDto;
use crate::model::dto::recipe::RecipeDto;
use crate::model::reactant::Reactant;
use crate::Recipe;

pub struct Factory {
    items: HashMap<String, Item>,
    buildings: HashMap<String, Building>,
}

impl Factory {
    pub fn into_items(self) -> HashMap<String,Item> {
        self.items
    }
}

impl Factory {
    pub fn create(book: &BookDto) -> Result<Self> {
        let buildings = book.buildings.iter()
            .map(|b| b.create_building())
            .map(|b: Building| (b.id().to_string(), b))
            .collect();

        let items:Result<HashMap<String,Item>> = book.items
            .iter()
            .map(|i| i.create_item(&buildings))
            .map(|r: Result<Item>| r.map(|i| (i.id().to_string(), i)))
            .collect();


        Ok(Factory { items:items?, buildings })
    }

    pub fn building(&self, building_id: &str) -> Result<Building> {
        self.buildings.get(building_id).cloned().ok_or_else(|| Error::UnknownBuilding(building_id.to_string()))
    }

    pub fn item(&self, item_id: &str) -> Result<Item> {
        self.items.get(item_id).cloned().ok_or_else(|| Error::UnknownItem(item_id.to_string()))
    }

    pub fn convert_recipe(&self, recipe: &RecipeDto) -> Result<Recipe> {
        let id = recipe.id.clone();
        let duration = recipe.duration;
        let building = self.building(&recipe.building)?;
        let alternate = recipe.alternate;
        let inputs: crate::error::Result<Vec<Reactant>> = recipe.inputs.iter().map(|r| self.convert_reactant(r)).collect();
        let outputs: crate::error::Result<Vec<Reactant>> = recipe.outputs.iter().map(|r| self.convert_reactant(r)).collect();

        Ok(Recipe::new(id, duration, building, alternate, inputs?, outputs?))
    }

    pub fn convert_reactant(&self, reactant: &ReactantDto) -> Result<Reactant> {
        let item = self.item(&reactant.item_id)?;
        Ok(Reactant::new(item, reactant.quantity))
    }

}