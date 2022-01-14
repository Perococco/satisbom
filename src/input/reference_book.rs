use crate::input::item::Item;
use serde::Deserialize;
use serde_json::Error;
use crate::input::building::Building;
use crate::input::recipe::Recipe;


#[derive(Deserialize,Debug)]
#[allow(dead_code)]
pub(crate) struct ReferenceBook {
    pub name:String,
    pub buildings:Vec<Building>,
    pub items:Vec<Item>,
    pub recipes:Vec<Recipe>
}

impl ReferenceBook {
    pub(crate) fn parse() -> Result<ReferenceBook,Error> {
        let book = include_str!("book_update5.json");
        serde_json::from_str(book)
    }
}



