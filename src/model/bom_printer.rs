use core::fmt;
use std::collections::HashMap;
use std::fmt::{Arguments, Error};
use std::fmt::Write;
use std::fs::File;

use hashlink::LinkedHashMap;
use term::color::Color;
use term::StdoutTerminal;

use crate::colors::{AMOUNT_COLOR, CONSTRUCTOR_COLOR, DURATION_COLOR, ITEM_COLOR, RECIPE_NAME_COLOR};
use crate::model::amount_format::AmountFormat;
use crate::model::building::Building;
use crate::model::item::Item;
use crate::model::reactant::Reactant;
use crate::Recipe;

pub struct BomPrinter<'a> {
    file: Option<File>,
    writer: Option<&'a mut dyn std::fmt::Write>,
    term: Option<Box<StdoutTerminal>>,
    amount_format: AmountFormat,
}

impl<'a> BomPrinter<'a> {
    pub fn fg(&mut self, color: Color) -> term::Result<()> {
        if let Some(t) = self.term.as_deref_mut() {
            t.fg(color)
        } else {
            Ok(())
        }
    }

    pub(crate) fn with_file(file:File, amount_format:AmountFormat) -> Self {
        BomPrinter{file:Some(file), writer:None, term:None, amount_format}
    }


    #[allow(dead_code)]
    pub(crate) fn with_writer(writer:&'a mut dyn std::fmt::Write, amount_format:AmountFormat) -> Self {
        BomPrinter{file:None, writer:Some(writer), term:None, amount_format}
    }


    pub fn with_term(amount_format: AmountFormat) -> Self {
        BomPrinter{file:None, writer: None, term: Some(term::stdout().unwrap()), amount_format }
    }

    pub fn reset(&mut self) -> term::Result<()> {
        if let Some(t) = self.term.as_deref_mut() {
            t.reset()
        } else {
            Ok(())
        }
    }
}

impl BomPrinter<'_> {
    pub fn display_buildings(&mut self, buildings: &HashMap<Building, u32>) -> crate::error::Result<()> {
        self.reset()?;
        writeln!(self, "=== Buildings ===")?;

        let mut total = 0;
        for (b, a) in buildings {
            let power_needed = b.power_usage() * (*a as i32);
            writeln!(self, "{:>8} - {:>13} ({:9} MW)", a, b.id(), power_needed)?;
            total += power_needed
        };

        writeln!(self, "{:>8}   {:>13} ({:9} MW)", "", "Total", total).map_err(crate::error::Error::Fmt)
    }

    pub fn display_recipes(&mut self, recipes: &LinkedHashMap<Recipe, f64>) -> crate::error::Result<()> {
        self.reset()?;
        writeln!(self, "=== Recipes ===")?;
        writeln!(self, "  {:>7} - {:<26} {:>3} {:>7} Detail", "#", "Name", "sec", "# Cons.")?;
        writeln!(self, "---------------------------------------------------------")?;

        for (recipe, amount) in recipes.iter() {
            let nb_need = amount / recipe.nb_per_minute();
            self.reset()?;
            write!(self, "  {:>7}", self.amount_format.format(amount))?;
            write!(self, " - ")?;
            self.fg(RECIPE_NAME_COLOR)?;
            write!(self, "{:<26}", recipe.id())?;
            self.fg(DURATION_COLOR)?;
            write!(self, " {:>3}", recipe.duration())?;
            self.fg(CONSTRUCTOR_COLOR)?;
            write!(self, " {:>7} ", self.amount_format.format(&nb_need))?;
            self.display_recipe(recipe,*amount)?;
            writeln!(self)?
        };

        Ok(())
    }

    pub fn display_items(&mut self, header: &str, items: &HashMap<Item, f64>) -> crate::error::Result<()> {
        if items.is_empty() {
            return Ok(());
        }

        self.reset()?;
        writeln!(self, "{}", header)?;
        for (item, amount) in items.iter() {
            write!(self, "{:>8} - {item}", self.amount_format.format(amount))?;
            writeln!(self)?;
        };

        Ok(())
    }

    pub fn display_recipe(&mut self, recipe: &Recipe, amount: f64) -> crate::error::Result<()> {
        for (i, reactant) in recipe.inputs().iter().enumerate() {
            if i != 0 {
                self.reset()?;
                write!(self, " + ")?;
            }
            self.display_reactant(reactant, amount)?;
        };

        self.reset()?;
        write!(self, " -> ")?;

        for (i, reactant) in recipe.outputs().iter().enumerate() {
            if i != 0 {
                self.reset()?;
                write!(self, " + ")?;
            }
            self.display_reactant(reactant, amount)?;
        };
        Ok(())
    }

    pub fn display_reactant(&mut self, reactant: &Reactant, amount: f64) -> crate::error::Result<()> {
        let quantity = amount * (reactant.quantity() as f64);
        self.fg(AMOUNT_COLOR)?;
        write!(self, "{}", self.amount_format.format(&quantity))?;
        self.reset()?;
        write!(self, "*")?;
        self.fg(ITEM_COLOR)?;
        write!(self, "{}", reactant.item())?;
        Ok(())
    }
}


impl std::fmt::Write for BomPrinter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use std::io::Write;
        if let Some(w) = self.writer.as_deref_mut() {
            w.write_str(s)
        } else if let Some(t) = self.term.as_deref_mut() {
            t.write_fmt(format_args!("{}", s)).map_err(|_| Error)
        } else if let Some(f) = self.file.as_mut() {
            f.write_fmt(format_args!("{}",s)).map_err(|_| Error)
        } else {
            Err(Error)
        }
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if let Some(w) = self.writer.as_deref_mut() {
            w.write_char(c)
        } else if let Some(t) = self.term.as_deref_mut() {
            t.write_fmt(format_args!("{}", c)).map_err(|_| Error)
        } else {
            Err(Error)
        }
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> fmt::Result {
        if let Some(w) = self.writer.as_deref_mut() {
            w.write_fmt(args)
        } else if let Some(t) = self.term.as_deref_mut() {
            t.write_fmt(args).map_err(|_| Error)
        } else {
            Err(Error)
        }
    }
}