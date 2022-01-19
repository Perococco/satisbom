use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use term::StdoutTerminal;
use crate::amount::{Amount, AmountF64, AmountRatio};
use crate::bag::{Bag, HashBag};
use crate::colors::{AMOUNT_COLOR, CONSTRUCTOR_COLOR, DEFAULT_COLOR, DURATION_COLOR, RECIPE_NAME_COLOR};
use crate::model::building::Building;
use crate::model::item::Item;
use crate::model::recipe::Recipe;

pub struct Bom<T> where T : Amount {
    pub targets:HashBag<Item,T>,
    pub requirements:HashBag<Item,T>,
    pub leftovers:HashBag<Item,T>,
    pub recipes:HashBag<Recipe, T>,
    pub buildings:HashBag<Building,T>
}


impl Bom<AmountF64> {

    pub fn new(targets:HashBag<Item,AmountF64>,
               requirements:HashBag<Item,AmountF64>,
               leftovers:HashBag<Item,AmountF64>,
               recipes:HashBag<Recipe,AmountF64>) -> Self {
        let mut buildings :HashBag<Building,AmountF64> = Default::default();

        let v:Vec<&str> = vec!["s"];
        v.iter();

        recipes.iter()
            .map(|(r, amount)| (r.building(), amount.per_minute(r.duration())))
            .fold(&mut buildings, |bag, (b, amount)| {
                bag.add(b.clone(), amount);
                bag
            }
            );

        buildings.clean();

        Bom{targets,requirements,leftovers,recipes,buildings}

    }
}


impl <T : Amount> Display for Bom<T>  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        for (recipe,amount) in self.recipes.iter() {
            if !amount.is_nil() {
                recipe.format(f,amount)?;
                f.write_char('\n')?;
            }
        };

        f.write_str("-----\n")?;
        for (building,amount) in self.buildings.iter() {
            f.write_fmt(format_args!("{:<22} {}\n",building.id(),amount))?;
        }

        Ok(())
    }
}

impl From<Bom<AmountF64>> for Bom<AmountRatio> {
    fn from(bom_f64: Bom<AmountF64>) -> Self {
        let buildings = bom_f64.buildings.into();
        let recipes:HashBag<Recipe,AmountRatio> = bom_f64.recipes.into();
        let targets = bom_f64.targets.into();
        let requirements = bom_f64.requirements.into();
        let leftovers = bom_f64.leftovers.into();


        Bom{targets,requirements,recipes,leftovers,buildings}
    }
}


fn display_item_list<T>(term:&mut StdoutTerminal, header:&str,items:&HashBag<Item,T>) -> crate::error::Result<()> where T:ToString {
    if items.is_empty() {
        return Ok(());
    }

    writeln!(term,"{}",header)?;
    for (item, amount) in items.iter() {
        write!(term,"{:>6} - {item}",amount.to_string())?;
        writeln!(term)?;
    };

    Ok(())
}


impl <T:Amount> Bom<T> {


    pub fn display(&self, term:&mut StdoutTerminal) -> crate::error::Result<()> {
        term.fg(DEFAULT_COLOR)?;

        display_item_list(term,"To get:",&self.targets);
        display_item_list(term,"You need:",&self.requirements);
        display_item_list(term,"Leftovers:",&self.leftovers);

        writeln!(term,"=== Recipes ===");
        writeln!(term,"  {:>7} - {:<26} {:>3} {:>7} {}", "#", "Name", "sec" ,"# Cons.","Detail")?;
        writeln!(term,"---------------------------------------------------------")?;

        for (recipe, amount) in self.recipes.iter() {
            let nb_need = amount.per_minute(recipe.duration());
            term.fg(DEFAULT_COLOR)?;
            write!(term,"  {:>7.2}",amount.to_string())?;
            write!(term," - ")?;
            term.fg(RECIPE_NAME_COLOR)?;
            write!(term,"{:<26}", recipe.id())?;
            term.fg(DURATION_COLOR)?;
            write!(term," {:>3}", recipe.duration())?;
            term.fg(CONSTRUCTOR_COLOR)?;
            write!(term," {:>7} ", nb_need.to_string())?;

            recipe.display(term, amount)?;
            writeln!(term)?
        }



        Ok(())
    }



}