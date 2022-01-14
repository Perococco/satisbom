use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum Building {
    Extractor(Extractor),
    Processor(Processor)
}



#[derive(Deserialize,Debug)]
pub struct Extractor {
    #[serde(rename(deserialize = "id"))]
    _id:String,
    #[serde(rename(deserialize = "type"))]
    _kind:String,
    #[serde(rename(deserialize = "power-usage"))]
    _power_usage:i32,
    #[serde(rename(deserialize = "power-extraction"))]
    _normal_extraction_rate:u32
}

#[derive(Deserialize,Debug)]
pub struct Processor {
    #[serde(rename(deserialize = "id"))]
    _id:String,
    #[serde(rename(deserialize = "type"))]
    _kind:String,
    #[serde(rename(deserialize = "power-usage"))]
    _power_usage:i32

}