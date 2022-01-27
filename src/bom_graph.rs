use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use dot::{Edges, GraphWalk, Id, Labeller, LabelText, Nodes, Style};
use dot::LabelText::LabelStr;
use dot::Style::{Filled};
use crate::model::item::Item;
use crate::{AmountFormat, Bom, Recipe};
use crate::constants::{is_nil, is_not_nil};
use crate::bom_graph::ItemType::{Available, Intermediate, LeftOver, Requirement, Target};


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ItemType {
    Intermediate,
    Available,
    Target,
    Requirement,
    LeftOver,
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
    amount_format: AmountFormat,
}

impl Graph {
    pub fn new(bom: &Bom, amount_format: AmountFormat) -> Self {
        let mut factory = GraphFactory::new(bom, amount_format);
        factory.build();
        factory.into()
    }
}


struct GraphFactory<'a> {
    bom: &'a Bom,

    node_index: HashMap<Node, usize>,
    nodes: Vec<Node>,
    edges: HashSet<(usize, usize)>,
    amount_format: AmountFormat,
}

impl From<GraphFactory<'_>> for Graph {
    fn from(factory: GraphFactory<'_>) -> Self {
        Graph { nodes: factory.nodes, edges: factory.edges.into_iter().collect(), amount_format: factory.amount_format }
    }
}

impl<'a> GraphFactory<'a> {
    fn new(bom: &'a Bom, amount_format: AmountFormat) -> Self {
        GraphFactory { bom, nodes: vec![], node_index: HashMap::new(), edges: HashSet::new(), amount_format }
    }
}

impl<'a> GraphFactory<'a> {
    fn build(&mut self) {
        self.create_all_recipe_nodes();

        let items = self.bom.get_all_items();

        for (item, (using, producing)) in items {
            let available_amount = self.bom.get_available_amount(item).cloned().unwrap_or(0f64);
            let target_amount = self.bom.get_targeted_amount(item).cloned().unwrap_or(0f64);
            let leftover_amount = self.bom.get_leftover_amount(item).cloned().unwrap_or(0f64);

            let produced_amount = producing.iter().map(|u| u.quantity).sum();
            let used_amount = using.iter().map(|u| u.quantity).sum();

            let is_produced = is_not_nil(produced_amount);
            let is_used = is_not_nil(used_amount);


            match (is_used, is_produced) {
                (true, true) => {
                    let node_index = self.add_item_node(item, produced_amount + available_amount, ItemType::Intermediate);
                    for iu in using {
                        let recipe_index = self.get_recipe_node_index(iu.recipe);
                        self.edges.insert((node_index,recipe_index));
                    }
                    for iu in producing {
                        let recipe_index = self.get_recipe_node_index(iu.recipe);
                        self.edges.insert((recipe_index,node_index));
                    }

                    if is_not_nil(target_amount) {
                        let target_node_index = self.add_item_node(item, target_amount, ItemType::Target);
                        self.edges.insert((node_index, target_node_index));
                    }
                    if is_not_nil(available_amount) {
                        let available_node_index = self.add_item_node(item, available_amount, ItemType::Available);
                        self.edges.insert((available_node_index, node_index));
                    }

                    if is_not_nil(leftover_amount) {
                        let leftover_node_index = self.add_item_node(item, leftover_amount, ItemType::LeftOver);
                        self.edges.insert((node_index, leftover_node_index));
                    }
                }
                (true, false) => {
                    let node_index = if is_nil(used_amount - available_amount) {
                        self.add_item_node(item, available_amount, Available)
                    } else {
                        let node_index = self.add_item_node(item, used_amount, Requirement);
                        if is_not_nil(available_amount) {
                            let available_index = self.add_item_node(item, available_amount, Available);
                            self.edges.insert((available_index, node_index));
                        }
                        node_index
                    };

                    for iu in using {
                        let recipe_index = self.get_recipe_node_index(iu.recipe);
                        self.edges.insert((node_index, recipe_index));
                    }
                }
                (false, true) => {
                    let node_index = match (is_not_nil(target_amount), is_not_nil(leftover_amount)) {
                        (false, false) => None,
                        (true, false) => {
                            let node_index = self.add_item_node(item, target_amount, Target);
                            Some(node_index)
                        }
                        (true, true) => {
                            let node_index = self.add_item_node(item, produced_amount, ItemType::Intermediate);
                            let target_idx = self.add_item_node(item, target_amount, Target);
                            let leftover_idx = self.add_item_node(item, leftover_amount, LeftOver);
                            self.edges.insert((node_index, target_idx));
                            self.edges.insert((node_index, leftover_idx));
                            Some(node_index)
                        }
                        (false, true) => {
                            let node_index = self.add_item_node(item, leftover_amount, LeftOver);
                            Some(node_index)
                        }
                    };

                    if let Some(node_index) = node_index {
                        for iu in producing {
                            let recipe_index = self.get_recipe_node_index(iu.recipe);
                            self.edges.insert((recipe_index, node_index));
                        }
                        if is_not_nil(available_amount) {
                            let available_node_index = self.add_item_node(item, available_amount, ItemType::Available);
                            self.edges.insert((node_index, available_node_index));
                        }
                    }
                }
                (false, false) => {
                    let node_index = match (is_not_nil(target_amount), is_not_nil(leftover_amount)) {
                        (false, false) => None,
                        (true, false) => {
                            let node_index = self.add_item_node(item, target_amount, Target);
                            Some(node_index)
                        }
                        (true, true) => {
                            let node_index = self.add_item_node(item, target_amount + leftover_amount, ItemType::Intermediate);
                            let target_idx = self.add_item_node(item, target_amount, Target);
                            let leftover_idx = self.add_item_node(item, leftover_amount, LeftOver);
                            self.edges.insert((node_index, target_idx));
                            self.edges.insert((node_index, leftover_idx));
                            Some(node_index)
                        }
                        (false, true) => {
                            let node_index = self.add_item_node(item, leftover_amount, LeftOver);
                            Some(node_index)
                        }
                    };

                    if let Some(node_index) = node_index {
                        for iu in producing {
                            let recipe_index = self.get_recipe_node_index(iu.recipe);
                            self.edges.insert((recipe_index, node_index));
                        }
                        if is_not_nil(available_amount) {
                            let available_node_index = self.add_item_node(item, available_amount, ItemType::Available);
                            self.edges.insert((node_index, available_node_index));
                        }
                    }
                }
            }
        }
    }

    fn create_all_recipe_nodes(&mut self) {
        for (recipe, amount) in &self.bom.recipes {
            self.add_recipe_node(recipe, *amount);
        }
    }
}

impl GraphFactory<'_> {
    fn add_node(&mut self, node:Node) -> usize {
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

    fn add_item_node(&mut self, item: &Item, amount: f64, item_type: ItemType) -> usize {
        let node = Node::Item(item.clone(), amount, item_type);
        self.add_node(node)
    }

    fn add_recipe_node(&mut self, recipe: &Recipe, amount: f64) -> usize {
        let node = Node::Recipe(recipe.clone(), amount);
        self.add_node(node)
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
        if let Node::Item(_, _, Intermediate) = _n {
            Style::Solid
        } else {
            Filled
        }
    }


    fn node_color(&'a self, _node: &Nd<'a>) -> Option<LabelText<'a>> {
        let color = match _node {
            Node::Recipe(_, _) => "#98B3FF",
            Node::Item(_, _, Target) => "#7EFF99",
            Node::Item(_, _, Requirement) => "#FF8075",
            Node::Item(_, _, Available) => "#FFD512",
            Node::Item(_, _, Intermediate) => "#000000",
            Node::Item(_, _, LeftOver) => "#DC14FF",
        };

        Some(LabelText::LabelStr(Cow::Borrowed(color)))
    }


    fn node_label(&'a self, n: &Nd<'a>) -> LabelText<'a> {
        let (name, a) = match n {
            Node::Recipe(r, a) => (r.id().replace("_", " "), a / r.nb_per_minute()),
            Node::Item(t, a, _) => (t.id().replace("_", " "), *a)
        };

        let label = format!("{}\n{}", name, self.amount_format.format(&a));


        LabelText::LabelStr(Cow::Owned(label))
    }

    fn edge_label(&'a self, e: &Ed) -> LabelText<'a> {
        let node0 = &self.nodes[e.0];
        let node1 = &self.nodes[e.1];

        match (node0, node1) {
            (Node::Item(item, item_amount, _), Node::Recipe(recipe, recipe_amount)) => {
                if let Some(re) = recipe.input_reactant(item) {
                    let consumed = re.quantity_f64() * recipe_amount;
                    let available = item_amount;
                    if is_nil(consumed - available) {
                        LabelStr(Cow::Borrowed(""))
                    } else {
                        LabelStr(Cow::Owned(format!("{:.2}", consumed)))
                    }
                } else {
                    LabelStr(Cow::Borrowed(""))
                }
            }
            (Node::Recipe(recipe, recipe_amount), Node::Item(item, item_amount, _)) => {
                if let Some(re) = recipe.output_reactant(item) {
                    let produced = re.quantity_f64() * recipe_amount;
                    let available = item_amount;
                    if is_nil(produced - available) {
                        LabelStr(Cow::Borrowed(""))
                    } else {
                        LabelStr(Cow::Owned(format!("{:.2}", produced)))
                    }
                } else {
                    LabelStr(Cow::Borrowed(""))
                }
            }
            (_, _) => LabelStr(Cow::Borrowed(""))
        }
    }
}
