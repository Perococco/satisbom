use std::collections::HashMap;
use serde::Deserialize;
use crate::model::building::Building;
use crate::model::item::{Item, Product, Resource};
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
    impure: Option<u32>,
    normal: Option<u32>,
    pure: Option<u32>,
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

        match extractor {
            Building::Extractor(e) => Ok(Resource::new(self.id.clone(), e.clone(), self.impure, self.normal, self.pure)),
            Building::Processor(_) => Err(Error::InvalidBuilding(self.extractor.to_string()))
        }
    }
}

impl ProductDto {
    fn create_product(&self) -> Result<Product> {
        Ok(Product::new(self.id.clone()))
    }
}
