use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct ReactantDto {
    #[serde(rename(deserialize = "item"))]
    pub item_id:String,
    #[serde(rename(deserialize = "quantity"))]
    pub quantity:u32,
}

