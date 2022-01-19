use serde::Deserialize;
use serde_json::Error;
use crate::{FullBook, Recipe};
use crate::model::dto::building::BuildingDto;
use crate::model::dto::factory::Factory;
use crate::model::dto::item::ItemDto;
use crate::model::dto::recipe::RecipeDto;
use crate::model::recipe_complexity::compute_complexity;


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

        let recipes: crate::error::Result<Vec<Recipe>> = self.recipes.iter()
            .map(|r| factory.convert_recipe(r))
            .collect();

        let mut recipes = recipes?;

        let complexities = compute_complexity(&recipes);


        recipes.sort_by(|r1, r2| complexities.get(r1.id()).cmp(&complexities.get(r2.id())));


        Ok(FullBook::new(factory.into_items(), recipes))
    }
}


#[cfg(test)]
mod tests {
    use crate::model::dto::book::BookDto;
    use crate::model::dto::factory::Factory;
    use crate::model::recipe_complexity::compute_complexity;
    use crate::Recipe;

    #[test]
    fn check_deserialization() {
        let book_dto = BookDto::parse();
        assert!(book_dto.is_ok())
    }

    #[test]
    fn test_complexity() {
        let book_dto = BookDto::parse().unwrap();

        let factory = Factory::create(&book_dto).unwrap();

        let recipes: crate::error::Result<Vec<Recipe>> = book_dto.recipes.iter()
            .map(|r| factory.convert_recipe(r))
            .collect();

        let mut recipes:Vec<Recipe> = recipes.unwrap()
            .into_iter().filter(|r| is_allowed(r))
            .collect();


        let complexities = compute_complexity(&recipes);
        assert_eq!(complexities.get("_iron_plate"),Some(&1));
        assert_eq!(complexities.get("_iron_rod"),Some(&1));
        assert_eq!(complexities.get("_screw"),Some(&2));

    }

    fn is_allowed(recipe: &Recipe) -> bool {
        match recipe.id() {
            "_iron_ingot" | "_iron_plate" | "_iron_rod" | "_screw" => true,
            _ => false
        }
    }
}



