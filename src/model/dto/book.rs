use serde::Deserialize;
use serde_json::Error;
use crate::{FullBook, Recipe};
use crate::model::dto::building::BuildingDto;
use crate::model::dto::factory::Factory;
use crate::model::dto::item::ItemDto;
use crate::model::dto::recipe::RecipeDto;


#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct BookDto {
    pub name: String,
    pub buildings: Vec<BuildingDto>,
    pub items: Vec<ItemDto>,
    pub recipes: Vec<RecipeDto>,
}

impl BookDto {
    pub(crate) fn parse() -> Result<BookDto, Error> {
        let book = include_str!("book_update5.json");
        serde_json::from_str(book)
    }
}

impl BookDto {
    pub(crate) fn to_full_book(&self) -> crate::error::Result<FullBook> {
        let factory = Factory::create(self)?;

        let recipes:crate::error::Result<Vec<Recipe>> = self.recipes.iter()
            .map(|r| factory.convert_recipe(r))
            .collect();


        Ok(FullBook::new(factory.to_items(), recipes?))
    }
}


#[cfg(test)]
mod tests {
    use crate::model::dto::book::BookDto;

    #[test]
    fn check_deserialization() {
        let book_dto = BookDto::parse();
        assert!(book_dto.is_ok())
    }
}



