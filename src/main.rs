extern crate core;

use std::collections::{HashMap, HashSet};
use std::fmt::{Display,  Formatter, Write};
use std::fs::{File, read_to_string};
use std::os::unix::raw::ino_t;
use std::str::FromStr;

use clap::{AppSettings, Parser};
use clap::ErrorKind::MissingRequiredArgument;
use maplit::hashmap;
use tempfile::{NamedTempFile};

use bom_graph::Graph;
use model::bom::Bom;
use model::book::FilterableBook;

use crate::error::{Error, Result};
use crate::Error::{Clap};
use crate::model::amount_format::AmountFormat;
use crate::model::bom_printer::BomPrinter;
use crate::model::full_book::FullBook;
use crate::model::recipe::Recipe;
use crate::model::recipe_complexity::sort_recipes;
use crate::problem_input::ProblemInput;
use crate::recipe_filter::RecipeFilter;
use crate::recipe_filter::RecipeFilter::{AllOf, NotAlternate, NotManual, NotNamed};

mod model;
mod problem_input;
mod error;
mod factory;
mod problem;
mod colors;
mod constants;
mod recipe_filter;
mod bom_graph;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Dump a template file that can be used by the bom subcommand
    Dump(DumpArg),
    /// Compute the BoM to produce some items
    Bom(BomArg),
    Search(SearchArgs),
}

#[derive(Parser, Debug)]
pub struct DumpArg {
    #[clap(short, long)]
    output_file: Option<String>,
}

#[derive(Parser, Debug)]
pub struct SearchArgs {
    pattern: String,
}

#[derive(Parser, Debug)]
pub struct BomArg {
    #[clap(short, long)]
    input_file: Option<String>,

    #[clap(short, long)]
    dump_file: Option<String>,

    #[clap(short, long)]
    filters: Option<String>,

    #[clap(short = 'F', default_value_t = Format::Text, arg_enum)]
    format: Format,

    #[clap(short = 'p', long)]
    //force printing the bom on the standard output if the -output-file option is used
    force_stdout: bool,

    #[clap(short, long)]
    output_file: Option<String>,

    #[clap(short, long)]
    use_ratio: bool,

    #[clap(short = 'w', long)]
    weight_by_abundance: Option<bool>,

    reactants: Vec<String>,

    #[clap(short = 'a', long)]
    available_items:String
}

impl BomArg {
    pub fn input_file(&self) -> &Option<String> {
        &self.input_file
    }
    pub fn reactants(&self) -> &Vec<String> {
        &self.reactants
    }
    pub fn format(&self) -> &Format {
        &self.format
    }
    pub fn use_ratio(&self) -> &bool {
        &self.use_ratio
    }
    pub fn filters(&self) -> &Option<String> {
        &self.filters
    }
    pub fn dump_file(&self) -> &Option<String> {
        &self.dump_file
    }
    pub fn weight_by_abundance(&self) -> Option<bool> {
        self.weight_by_abundance
    }

    fn parsed_filters(&self) -> Result<Option<RecipeFilter>> {
        self.filters.as_ref()
            .map(|f| parse_filter(f))
            .map(|r| r.map(Some))
            .unwrap_or(Ok(None))
    }

    fn parsed_reactants(&self) -> Result<HashMap<String, u32>> {
        self.reactants.iter()
            .map(|r| r.parse::<InputItem>())
            .map(|r| r.map(|i| (i.name, i.quantity)))
            .collect::<Result<HashMap<String, u32>>>()
    }

    fn parsed_available_items(&self) -> Result<HashMap<String, u32>> {
        self.available_items
            .split(",")
            .map(|r| r.parse::<InputItem>())
            .map(|r| r.map(|i| (i.name, i.quantity)))
            .collect::<Result<HashMap<String, u32>>>()
    }


    pub fn force_stdout(&self) -> bool {
        self.force_stdout
    }
    pub fn output_file(&self) -> &Option<String> {
        &self.output_file
    }
    pub fn available_items(&self) -> &String {
        &self.available_items
    }
}

#[derive(Debug, clap::ArgEnum, Clone)]
pub enum Format {
    Text,
    Dot,
    Png,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Dot => write!(f, "dot"),
            Format::Png => write!(f, "png"),
        }
    }
}

impl FromStr for Format {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "txt" => Ok(Format::Text),
            "dot" => Ok(Format::Dot),
            _ => Err(std::fmt::Error)
        }
    }
}


fn main() -> crate::error::Result<()> {
    let args: Args = Args::parse();

    match args.command {
        Command::Dump(d) => dump(d),
        Command::Bom(b) => bom(b),
        Command::Search(s) => search(s)
    }
}

pub struct InputItem {
    pub name: String,
    pub quantity: u32,
}

impl FromStr for InputItem {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (qty, name) = s.split_once(".").ok_or_else(|| Error::TargetParsingFailed(s.to_string()))?;
        let quantity = qty.parse::<u32>().map_err(|_| Error::TargetParsingFailed(s.to_string()))?;
        Ok(InputItem { name: name.to_string(), quantity })
    }
}

fn bom(args: BomArg) -> crate::error::Result<()> {
    if args.input_file().is_none() && args.reactants().is_empty() {
        return Err(Clap(MissingRequiredArgument));
    };

    let mut input = args.input_file()
        .as_ref()
        .map(|f| read_input(f))
        .unwrap_or_else(|| Ok(ProblemInput::default()))?;

    let available_items = args.parsed_available_items()?;
    let reactants = args.parsed_reactants()?;
    let filters = args.parsed_filters()?;

    if let Some(ua) = args.weight_by_abundance() {
        input.use_abundances = ua;
    }

    if !reactants.is_empty() {
        input.target_items = reactants;
    }

    if !available_items.is_empty() {
        input.available_items = available_items;
    }

    if let Some(filter) = filters {
        input.filter = filter
    }


    if let Some(dump_file) = args.dump_file().as_ref() {
        let file = File::create(dump_file)?;
        serde_json::to_writer_pretty(file, &input)?;
        Ok(())
    } else {
        let bom = Bom::optimized(&input)?;

        let amount_format = if *args.use_ratio() { AmountFormat::Ratio } else { AmountFormat::F64 };

        if args.output_file().is_some() && args.force_stdout() {
            let mut printer = BomPrinter::with_term(amount_format);
            bom.display(&mut printer)?;
        }


        match &args.format {
            Format::Text => {
                let mut printer = if let Some(f) = args.output_file() {
                    BomPrinter::with_file(File::create(format!("{}.txt", f))?, amount_format)
                } else {
                    BomPrinter::with_term(amount_format)
                };

                bom.display(&mut printer)
            }
            Format::Dot => {
                let graph: Graph = Graph::new(&bom, amount_format);

                if let Some(f) = args.output_file() {
                    let mut file = File::create(format!("{}.dot", f))?;
                    dot::render(&graph, &mut file)?;
                } else {
                    dot::render(&graph, &mut std::io::stdout())?;
                };


                Ok(())
            }
            Format::Png => {
                let graph: Graph = Graph::new(&bom, amount_format);
                let named_file = NamedTempFile::new()?;
                dot::render(&graph, &mut named_file.as_file())?;

                let output = std::process::Command::new("dot")
                    .arg("-Tpng")
                    .arg(named_file.path())
                    .output()?;

                if output.status.success() {
                    use std::io::Write;
                    if let Some(f) = args.output_file() {
                        let mut file = File::create(format!("{}.png", f))?;
                        file.write_all(&output.stdout)?;
                    } else {
                        std::io::stdout().write_all(&output.stdout)?;
                    };
                    Ok(())
                } else {
                    Err(Error::DotFailed)
                }
            }
        }
    }
}

fn parse_filter(filter_str: &str) -> crate::error::Result<RecipeFilter> {
    let mut filters = vec![];
    for filter in filter_str.split(',') {
        let f = filter.parse::<RecipeFilter>()?;
        filters.push(f)
    }

    Ok(AllOf(filters))
}

fn read_input(input_file: &str) -> crate::error::Result<ProblemInput> {
    let content = read_to_string(input_file)?;
    let input = serde_json::from_str::<ProblemInput>(&content)?;
    Ok(input)
}

fn dump(args: DumpArg) -> crate::error::Result<()> {
    let mut names = HashSet::new();
    names.insert("copper_ingot".to_string());
    names.insert("caterium_ingot".to_string());

    let input = ProblemInput {
        target_items: hashmap! {
             "iron_plate".to_string() => 30,
             "iron_rod".to_string() => 30,
        },
        available_items: hashmap! {
            "iron_ingot".to_string() => 30,
        },
        use_abundances: true,
        filter: AllOf(vec![NotAlternate, NotManual, NotNamed(names)]),
    };

    match args.output_file {
        None => {
            let result = serde_json::to_string_pretty(&input)?;
            println!("{}", result);
            Ok(())
        }
        Some(file_name) => {
            let file = File::create(file_name)?;
            serde_json::to_writer_pretty(file, &input)?;
            Ok(())
        }
    }
}


fn search(search_args: SearchArgs) -> crate::error::Result<()> {
    let pattern = search_args.pattern;
    let book = FullBook::create()?;

    let recipes: Vec<Recipe> = book.recipes().iter().filter(|r| r.id().contains(&pattern)).cloned().collect();
    let recipes = sort_recipes(recipes);

    let mut writer = BomPrinter::with_term(AmountFormat::F64);

    let mut should_display_header = true;
    for recipe in recipes {
        if recipe.id().contains(&pattern) {
            if should_display_header {
                writer.reset()?;
                should_display_header = false;
                writeln!(writer, "=== Recipe ===")?;
            }
            writer.reset()?;
            write!(writer, "{:<25}  : ", recipe.id())?;
            writer.display_recipe(&recipe, 1f64)?;
            writeln!(writer)?;
        }
    }

    let mut should_display_header = true;
    for item in book.items().keys() {
        if item.contains(&pattern) {
            if should_display_header {
                should_display_header = false;
                writer.reset()?;
                writeln!(writer, "=== Items ===")?;
            }
            writeln!(writer, " {}", item)?;
        }
    }

    Ok(())
}
