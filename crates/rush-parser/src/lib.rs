use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Relation {
    SingleChoice,
    Optional,
    Reset,
    Path,
}

pub trait IntoString: Into<String> + std::fmt::Display + Clone {}
impl<T: Into<String> + std::fmt::Display + Clone> IntoString for T {}

lazy_static! {
    static ref OPERATOR_MAP: HashMap<char, Relation> = HashMap::from([
        // Map contains the operator and the relation it represents
        ('^', Relation::SingleChoice), // Only one of the variants may exist
        ('!', Relation::Optional), // The expression is optional, it may or may not exist
        ('@', Relation::Reset)
    ]);
}

#[derive(Debug, Builder, PartialEq, Eq)]
#[builder(pattern = "owned")]
#[builder(name = "NodeBuilder")]
pub struct TreeNode {
    value: String,
    relation: Relation,

    #[builder(default = "HashMap::new()")]
    joint_nodes: HashMap<String, Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn insert<S: IntoString>(&mut self, id: S, node: Rc<RefCell<TreeNode>>) {
        self.joint_nodes.insert(id.into(), node);
    }
    pub fn relation(&mut self, relation: Relation) {
        self.relation = relation;
    }
}

fn get_node_from_value<'a, S: IntoString>(value: S, relation: Relation) -> Result<Rc<RefCell<TreeNode>>, String> {
    let mut value = value.into();
    if value.ends_with("+r") {
        value.pop();
        value.pop();
        let node = generate_relation_tree(&value)?;
        node.borrow_mut().relation = relation;
        return Ok(node);
    }
    return Ok(Rc::from(RefCell::from(
        NodeBuilder::default()
            .value(value)
            .relation(relation)
            .build()
            .unwrap(),
    )));
}

fn print_tree(root_node: &TreeNode, ident_level: u8, visited: &mut Vec<TreeNode>) {
    let mut ident_string = String::new();
    for _ in 0..ident_level {
        ident_string.push(' ');
    }
    let ident_level = ident_level + 4;
    println!(
        "{}{} {:?}",
        ident_string, root_node.value, root_node.relation
    );
    for _ in 0..ident_level / 2 {
        ident_string.push(' ');
    }
    for (_, node) in &root_node.joint_nodes {
        if visited
            .iter()
            .find(|fnode| *fnode == &*node.borrow())
            .is_none()
        {
            visited.push(TreeNode {
                value: node.borrow().value.clone(),
                joint_nodes: node.borrow().joint_nodes.clone(),
                relation: node.borrow().relation,
            });
            print_tree(&*node.borrow(), ident_level, visited);
        } else {
            println!(
                "{}{} {:?}",
                ident_string,
                node.borrow().value,
                node.borrow().relation
            );
        }
    }
}

#[allow(unused_variables, unused_mut, unused_assignments)]
fn generate_relation_tree<S: IntoString>(stream: S) -> Result<Rc<RefCell<TreeNode>>, String> 
{
    let stream = stream.into();
    if stream.is_empty() {
        return Err("Empty string cannot be parsed".to_owned());
    }
    let split_stream: Vec<&str> = stream.split_whitespace().collect();

    let tree_root = Rc::new(RefCell::new(TreeNode {
        value: split_stream[0].to_string(),
        relation: Relation::Path,
        joint_nodes: HashMap::new(),
    }));

    let mut current_node = Rc::clone(&tree_root);

    let mut split_stream = if split_stream.len() > 1 {
        split_stream[1..].to_vec()
    } else {
        Vec::new()
    };
    if !split_stream.is_empty() {
        split_stream.push(" ")
    }
    let joint_stream = split_stream.join(" ");

    {
        let mut optional = false;
        let mut oneselect = false;
        let mut reset = false;
        let mut dump_stack = String::new();
        let mut operation_stack: Vec<String> = Vec::new();
        let mut bcount = 0;
        for ch in joint_stream.chars() {
            if ch == '(' {
                if bcount > 0 {
                    dump_stack.push(ch);
                }
                bcount += 1;
                continue;
            }
            if bcount > 0 {
                if ch == ')' {
                    bcount -= 1
                }
                if bcount > 0 {
                    dump_stack.push(ch);
                } else {
                    dump_stack.push_str("+r");
                }
                continue;
            }
            if let Some(relation) = OPERATOR_MAP.get(&ch) {
                match relation {
                    Relation::Optional => optional = true,
                    Relation::Reset => reset = true,
                    Relation::SingleChoice => {
                        if dump_stack.is_empty() {
                            return Err("No first operand found for operator ^".into());
                        }
                        operation_stack.push(dump_stack.clone());
                        dump_stack.clear();
                        oneselect = true;
                    }
                    _ => {}
                }
            } else if ch == ' ' || ch == '\t' {
                if !dump_stack.is_empty() {
                    if oneselect {
                        operation_stack.push(dump_stack.clone());
                        for operand in &operation_stack {
                            let node =
                                get_node_from_value(operand.clone(), Relation::SingleChoice)?;
                            current_node
                                .borrow_mut()
                                .insert(operand.clone(), Rc::clone(&node));
                        }
                        oneselect = false;
                    } else if reset {
                        current_node
                            .borrow_mut()
                            .insert(dump_stack.clone(), Rc::clone(&tree_root));
                        reset = false;
                    } else if optional {
                        current_node.borrow_mut().insert(
                            dump_stack.clone(),
                            get_node_from_value(dump_stack.clone(), Relation::Optional)?,
                        );
                        optional = false;
                    } else {
                        let node = get_node_from_value(dump_stack.clone(), Relation::Path)?;
                        current_node
                            .borrow_mut()
                            .insert(dump_stack.clone(), Rc::clone(&node));
                        current_node = node;
                    }
                    dump_stack.clear();
                }
            } else {
                dump_stack.push(ch);
            }
        }
        if !dump_stack.is_empty() {
            current_node.borrow_mut().insert(
                dump_stack.clone(),
                Rc::from(RefCell::from(
                    NodeBuilder::default()
                        .value(dump_stack.clone())
                        .relation(if optional {
                            Relation::Optional
                        } else {
                            Relation::Path
                        })
                        .build()
                        .unwrap(),
                )),
            )
        }
    }

    Ok(tree_root)
}

pub mod analyzer {
    use crate::{generate_relation_tree, print_tree, TreeNode, IntoString};

    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    pub struct SyntaxValidationTree {
        entry_points: HashMap<&'static str, Rc<RefCell<TreeNode>>>,
        pub current: Option<Rc<RefCell<TreeNode>>>,
        pub parent: Option<Rc<RefCell<TreeNode>>>,
    }

    impl SyntaxValidationTree {
        pub fn from(syntax_streams: Vec<(&'static str, &'static str)>) -> Self {
            let mut syntax_tree = Self {
                entry_points: HashMap::new(),
                current: None,
                parent: None,
            };
            for (entry, stream) in syntax_streams {
                syntax_tree
                    .entry_points
                    .insert(entry, generate_relation_tree(stream).unwrap());
            }
            syntax_tree
        }

        pub fn show_entry<'a, S: IntoString>(&self, entry: S) -> Result<(), String>
        {
            if let Some(tree) = self.entry_points.get(entry.clone().into().as_str()) {
                println!("Showing for entry: {}", entry);
                print_tree(&*tree.borrow(), 0, &mut vec![]);
            } else {
                return Err(format!("Entry {} does not exist", entry));
            }
            Ok(())
        }

        pub fn set_current<S: IntoString>(&mut self, entry: S) -> Result<(), String>
        {
            if self.current.is_none() {
                if let Some(node) = self.entry_points.get(entry.clone().into().as_str()) {
                    self.current = Some(Rc::clone(node));
                    Ok(())
                } else {
                    Err(format!("Entry: {} does not exist", entry))
                }
            } else {
                let a_node;
                {
                    match self
                        .current
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .joint_nodes
                        .get(entry.clone().into().as_str()) {
                        Some(node) => {
                            a_node = Some(Rc::clone(node));
                        }
                        None => {
                            return Err(format!("Entry: {} does not exist", entry));
                        }
                    }
                }
                self.current = a_node;
                Ok(())
            }
        }

        pub fn set_parent(&mut self) {
            self.parent = self.current.clone();
        }

        pub fn reset_parent(&mut self) {
            self.parent = None;
        }

        pub fn entry_exists<S: IntoString>(&self, entry: S) -> bool {
            if *&self.current.is_some() {
                self.current
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .joint_nodes
                    .get(entry.into().as_str())
                    .is_some()
            } else {
                self.entry_points.get(entry.into().as_str()).is_some()
            }
        }
        pub fn entries(&self) -> Vec<String> {
            let mut entrylist = Vec::new();
            for (entry,_) in &self.entry_points {
                entrylist.push(entry.to_string());
            }
            entrylist
        }
    }

}
