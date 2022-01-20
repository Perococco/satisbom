use std::collections::HashMap;
use serde::Deserialize;
use crate::model::building::Building;
use crate::model::item::{Item, Nodes, Product, Resource};
use crate::error::{Error,Result};


#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ItemDto {
    Resource(ResourceDto),
    Product(ProductDto),
}

#[derive(Deserialize, Debug)]
pub struct ProductDto {
    id: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ResourceDto {
    id: String,
    extractor: String,
    impure: i32,
    normal: i32,
    pure: i32,
}


impl ItemDto {

    pub fn create_item(&self, buildings:&HashMap<String,Building>) -> Result<Item> {
        match self {
            ItemDto::Resource(r) => r.create_resource(buildings).map(Item::Resource),
            ItemDto::Product(p) => p.create_product().map(Item::Product)
        }
    }
}


impl ResourceDto {
    fn create_resource(&self, buildings:&HashMap<String,Building>) -> Result<Resource> {
        let id = if self.extractor.eq("miner") {"miner_mk1"} else {&self.extractor};

        let extractor = buildings.get(id).ok_or_else(|| Error::UnknownBuilding(self.extractor.to_string()))?;

        let impure = to_u32(self.impure);
        let normal = to_u32(self.normal);
        let pure = to_u32(self.pure);

        let total = impure + normal + pure;

        let nodes = if total == 0 {None} else {Some(Nodes::new(impure,normal,pure))};

        match extractor {
            Building::Extractor(e) => Ok(Resource::new(self.id.clone(), e.clone(), nodes)),
            Building::Processor(_) => Err(Error::InvalidBuilding(self.extractor.to_string()))
        }
    }
}

fn to_u32(value:i32) -> u32 {
    if value <= 0 {
        0
    } else {
        value as u32
    }
}

impl ProductDto {
    fn create_product(&self) -> Result<Product> {
        Ok(Product::new(self.id.clone()))
    }
}
