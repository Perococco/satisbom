use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum Item {
    Resource(Resource),
    Product(Product),
}

#[derive(Deserialize,Debug)]
pub struct Product {
    id:String
}

#[derive(Deserialize,Debug)]
#[allow(dead_code)]
pub struct Resource {
    id:String,
    extractor:String,
    impure:Option<u32>,
    normal:Option<u32>,
    pure:Option<u32>,
}

impl Item {
    pub fn get_id(&self) -> &str {
        match self {
            Item::Resource(r) => &r.id,
            Item::Product(i) => &i.id
        }
    }
}