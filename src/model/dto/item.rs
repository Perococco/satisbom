use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum ItemDto {
    Resource(ResourceDto),
    Product(ProductDto),
}

#[derive(Deserialize,Debug)]
pub struct ProductDto {
    id:String
}

#[derive(Deserialize,Debug)]
#[allow(dead_code)]
pub struct ResourceDto {
    id:String,
    extractor:String,
    impure:Option<u32>,
    normal:Option<u32>,
    pure:Option<u32>,
}

impl ItemDto {
    pub fn get_id(&self) -> &str {
        match self {
            ItemDto::Resource(r) => &r.id,
            ItemDto::Product(i) => &i.id
        }
    }
}