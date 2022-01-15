use std::fmt::Formatter;
use crate::model::item::Item;

#[derive(Clone)]
pub struct Reactant {
    item:Item,
    quantity:u32,
}

impl Reactant {

    pub fn item(&self) -> &Item {
        &self.item
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn new(item: Item, quantity: u32) -> Self {
        Reactant { item, quantity }
    }

}

impl Reactant {

    pub fn format(&self, f:&mut Formatter<'_>, amount:f64) -> std::fmt::Result {
        let quantity = (self.quantity as f64) * amount;
        f.write_fmt(format_args!("{:.4}x{}", quantity, self.item))
    }

}