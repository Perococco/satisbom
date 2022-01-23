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

    pub fn item_id(&self) -> &str {
        self.item.id()
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn quantity_f64(&self) -> f64 {
        self.quantity as f64
    }

    pub fn new(item: Item, quantity: u32) -> Self {
        Reactant { item, quantity }
    }

}

impl Reactant {

    pub fn format(&self, f:&mut Formatter<'_>, amount:f64) -> std::fmt::Result {
        let quantity = amount * (self.quantity as f64);
        f.write_fmt(format_args!("{:.4}x{}", quantity, self.item))
    }





}