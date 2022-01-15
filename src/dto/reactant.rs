use std::fmt::Formatter;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Reactant {
    #[serde(rename(deserialize = "item"))]
    item_id:String,
    #[serde(rename(deserialize = "quantity"))]
    quantity:u32,
}

impl Reactant {

    pub fn item_id(&self) -> &str {
        &self.item_id
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }
}

impl Reactant {
    pub fn format(&self, f:&mut Formatter<'_>, amount:f64) -> std::fmt::Result {
        let quantity = (self.quantity as f64) * amount;
        f.write_fmt(format_args!("{:.4}x{}", quantity, self.item_id))
    }

}