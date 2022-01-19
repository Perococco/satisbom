use std::fmt::Formatter;
use term::StdoutTerminal;

use crate::amount::Amount;
use crate::colors::{AMOUNT_COLOR, DEFAULT_COLOR, ITEM_COLOR};
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

    pub fn format<T>(&self, f:&mut Formatter<'_>, amount:&T) -> std::fmt::Result where T:Amount {
        let quantity = amount.multiply(self.quantity);
        f.write_fmt(format_args!("{:.4}x{}", quantity, self.item))
    }


    pub fn display<T>(&self, term:&mut StdoutTerminal, amount:&T) -> crate::error::Result<()> where T:Amount {
        let quantity = amount.multiply(self.quantity);
        term.fg(AMOUNT_COLOR)?;
        write!(term,"{}",quantity)?;
        term.fg(DEFAULT_COLOR)?;
        write!(term,"*")?;
        term.fg(ITEM_COLOR)?;
        write!(term,"{}",self.item)?;
        Ok(())
    }



}