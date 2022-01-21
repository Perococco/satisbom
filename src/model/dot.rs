use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::path::Display;
use std::ptr::addr_of;
use dot::{Edges, GraphWalk, Id, Labeller, LabelText, Nodes, Style};
use dot::LabelText::LabelStr;
use dot::Style::{Filled, Solid};
use crate::model::item::Item;
use crate::{Bom, Recipe};
use crate::model::dot::ItemType::{Intermediate, Required, Targeted};
use crate::model::reactant::Reactant;


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ItemType {
    Intermediate,
    Targeted,
    Required,
}


#[derive(Clone)]
pub enum Node {
    Item(Item, f64, ItemType),
    Recipe(Recipe, f64),
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Item(i1, _, t1), Node::Item(i2, _, t2)) => i1.id().eq(i2.id()) && t1.eq(t2),
            (Node::Recipe(r1, _), Node::Recipe(r2, _)) => r1.id().eq(r2.id()),
            (_, _) => false
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Node::Item(item, _, item_type) => {
                item.id().hash(state);
                item_type.hash(state);
                state.write_i8(0)
            }
            Node::Recipe(recipe, _) => {
                recipe.id().hash(state);
                state.write_i8(1)
            }
        }
    }
}


pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn new(bom: &Bom) -> Self {
        let mut factory = GraphFactory::new(bom);
        factory.build();
        factory.into()
    }
}


struct GraphFactory<'a> {
    bom: &'a Bom,
    recipes_by_input_items: HashMap<Item, Vec<Recipe>>,

    node_index: HashMap<Node, usize>,
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl From<GraphFactory<'_>> for Graph {
    fn from(factory: GraphFactory<'_>) -> Self {
        Graph { nodes: factory.nodes, edges: factory.edges }
    }
}

impl<'a> GraphFactory<'a> {
    fn new(bom: &'a Bom) -> Self {
        let recipes_by_input_items = bom.get_recipes_by_input_item();
        GraphFactory { bom, nodes: vec![], node_index: HashMap::new(), edges: vec![], recipes_by_input_items }
    }
}

impl<'a> GraphFactory<'a> {
    fn build(&mut self) {
        self.create_all_recipe_nodes();
        for (recipe, amount) in &self.bom.recipes {
            self.handle_one_recipe(recipe, amount);
        };

        self.handle_requirements();
    }

    fn create_all_recipe_nodes(&mut self) {
        for (recipe, amount) in &self.bom.recipes {
            self.add_node(Node::Recipe(recipe.clone(), *amount));
        }
    }

    fn handle_one_recipe(&mut self, recipe: &Recipe, amount: &f64) {
        let recipe_node_index = self.get_recipe_node_index(recipe);

        for output in recipe.outputs() {
            let amount = (output.quantity() as f64) * amount;
            self.handle_one_output(output.item(), amount, recipe_node_index);
        }
    }

    fn get_recipes_using(&self, item: &Item) -> Option<Vec<Recipe>> {
        self.recipes_by_input_items.get(item).cloned()
    }

    fn handle_one_output(&mut self, item: &Item, amount: f64, recipe_node_index: usize) {
        let targeted_amount = self.bom.get_targeted_amount(item);

        let node_index = if let Some(recipe_using) = self.get_recipes_using(item) {
            let node = Node::Item(item.clone(), amount, Intermediate);
            let index = self.add_node(node);

            self.edges.push((recipe_node_index, index));
            for recipe in recipe_using {
                let recipe_index = self.get_recipe_node_index(&recipe);
                self.edges.push((index, recipe_index))
            }

            Some(index)
        } else {
            None
        };


        if let Some(ta) = targeted_amount {
            let target_node = Node::Item(item.clone(), *ta, Targeted);
            let target_node_index = self.add_node(target_node);

            match node_index {
                Some(idx) => self.edges.push((idx, target_node_index)),
                None => self.edges.push((recipe_node_index, target_node_index))
            }
        };
    }

    fn handle_requirements(&mut self) {
        for (item, amount) in &self.bom.requirements {
            let node_index = self.add_node(Node::Item(item.clone(), *amount, Required));

            if let Some(recipes) = self.recipes_by_input_items.get(item) {
                for recipe in recipes {
                    let recipe_index = self.get_recipe_node_index(recipe);
                    self.edges.push((node_index, recipe_index))
                }
            }
        }
    }
}

impl GraphFactory<'_> {
    fn add_node(&mut self, node: Node) -> usize {
        let index = self.node_index.get(&node);
        match index {
            Some(i) => *i,
            None => {
                let index = self.nodes.len();
                self.nodes.push(node.clone());
                self.node_index.insert(node, index);
                index
            }
        }
    }

    fn get_recipe_node_index(&self, recipe: &Recipe) -> usize {
        let node = Node::Recipe(recipe.clone(), 0f64);
        *(self.node_index.get(&node).unwrap())
    }
}

type Nd<'a> = &'a Node;
type Ed = (usize, usize);


impl<'a> GraphWalk<'a, Nd<'a>, Ed> for Graph {
    fn nodes(&'a self) -> Nodes<'a, Nd<'a>> {
        let vec_of_ref = self.nodes.iter().collect();
        Cow::Owned(vec_of_ref)
    }

    fn edges(&'a self) -> Edges<'a, Ed> {
        Cow::Owned(self.edges.clone())
    }

    fn source(&'a self, edge: &Ed) -> Nd<'a> {
        &self.nodes[edge.0]
    }

    fn target(&'a self, edge: &Ed) -> Nd<'a> {
        &self.nodes[edge.1]
    }
}

impl<'a> Labeller<'a, Nd<'a>, Ed> for Graph {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("BOM").unwrap()
    }

    fn node_id(&'a self, n: &Nd<'a>) -> Id<'a> {
        let id = match n {
            Node::Item(item, _, t) => format!("{}_{:?}", item.id(), t),
            Node::Recipe(recipe, _) => recipe.id().to_string(),
        };
        Id::new(id).unwrap()
    }

    fn node_shape(&'a self, _node: &Nd<'a>) -> Option<LabelText<'a>> {
        Some(LabelStr(Cow::Borrowed("box")))
    }


    fn node_style(&'a self, _n: &Nd<'a>) -> Style {
        if let Node::Item(_, _,Intermediate) = _n {
            Style::Solid
        } else {
            Filled
        }
    }


    fn node_color(&'a self, _node: &Nd<'a>) -> Option<LabelText<'a>> {
        let color = match _node {
            Node::Recipe(_, _) => "#98B3FF",
            Node::Item(_, _, Targeted) => "#7EFF99",
            Node::Item(_, _, Required) => "#FF8075",
            Node::Item(_, _, Intermediate) => "#000000",
        };

        Some(LabelText::LabelStr(Cow::Borrowed(color)))
    }


    fn node_label(&'a self, n: &Nd<'a>) -> LabelText<'a> {
        let (name,a) = match n {
            Node::Recipe(r, a) => (r.id().replace("_"," "),a),
            Node::Item(t, a,_) => (t.id().replace("_"," "),a)
        };

        let label = format!("{}\n{:.3}",name,a);


        LabelText::LabelStr(Cow::Owned(label))

    }


}
