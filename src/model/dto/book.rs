use crate::dto::item::ItemDto;
use serde::Deserialize;
use serde_json::Error;
use crate::dto::building::BuildingDto;
use crate::dto::recipe::RecipeDto;


#[derive(Deserialize,Debug)]
#[allow(dead_code)]
pub(crate) struct BookDto {
    pub name:String,
    pub buildings:Vec<BuildingDto>,
    pub items:Vec<ItemDto>,
    pub recipes:Vec<RecipeDto>
}

impl BookDto {
    pub(crate) fn parse() -> Result<BookDto,Error> {
        let book = include_str!("book_update5.json");
        serde_json::from_str(book)
    }
}

#[cfg(test)]
mod tests {
    use crate::BookDto;

    #[test]
    fn check_deserialization() {
        let book_dto = BookDto::parse();
        assert!(book_dto.is_ok())
    }
}



