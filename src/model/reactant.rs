use std::fmt::Formatter;
use std::fmt::Write;

use crate::colors::{AMOUNT_COLOR, DEFAULT_COLOR, ITEM_COLOR};
use crate::model::bom_printer::BomPrinter;
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

    pub fn new(item: Item, quantity: u32) -> Self {
        Reactant { item, quantity }
    }

}

impl Reactant {

    pub fn format(&self, f:&mut Formatter<'_>, amount:f64) -> std::fmt::Result {
        let quantity = amount * (self.quantity as f64);
        f.write_fmt(format_args!("{:.4}x{}", quantity, self.item))
    }


    pub fn display(&self, term:&mut BomPrinter, amount:f64) -> crate::error::Result<()> {
        let quantity = amount * (self.quantity as f64);
        term.fg(AMOUNT_COLOR)?;
        write!(term,"{}",quantity)?;
        term.fg(DEFAULT_COLOR)?;
        write!(term,"*")?;
        term.fg(ITEM_COLOR)?;
        write!(term,"{}",self.item)?;
        Ok(())
    }



}