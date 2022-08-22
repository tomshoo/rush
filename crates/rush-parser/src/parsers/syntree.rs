use crate::IntoString;
use derive_builder::*;
use lazy_static::lazy_static;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

// Tree path relations
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Relation {
    SingleChoice,
    Optional,
    Reset,
    Path,
}

lazy_static! {
    static ref OPERATOR_MAP: HashMap<char, Relation> = HashMap::from([
        // Map contains the operator and the relation it represents
        ('^', Relation::SingleChoice), // Only one of the variants may exist
        ('!', Relation::Optional), // The expression is optional, it may or may not exist
        ('@', Relation::Reset)
    ]);
}

// Node wrapper for the syntax validation tree
#[derive(Debug, Builder, PartialEq, Eq)]
#[builder(pattern = "owned")]
#[builder(name = "NodeBuilder")]
pub(super) struct TreeNode {
    value: String,
    relation: Relation,

    #[builder(default = "HashMap::new()")]
    joint_nodes: HashMap<String, Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    // Add new node
    pub fn insert<S: IntoString>(&mut self, id: S, node: Rc<RefCell<TreeNode>>) {
        self.joint_nodes.insert(id.into(), node);
    }

    // Set relation for current node
    pub fn relation(&mut self, relation: Relation) {
        self.relation = relation;
    }

    pub fn get<S: IntoString>(&self, key: S) -> Option<Ref<Self>> {
        let entry = self.joint_nodes.get(&key.into())?;
        Some(entry.borrow())
    }
}

// Evaluate node, indirect recursion if requested
// Recursion request is made by appending "+r" at the end of evaluation string
fn get_node_from_value<'a, S: IntoString>(
    value: S,
    relation: Relation,
) -> Result<Rc<RefCell<TreeNode>>, String> {
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

// Print the tree from the given node
// Utilizes an immensly simplified DFS approach
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

// Generate the relation tree
#[allow(unused_variables, unused_mut, unused_assignments)]
fn generate_relation_tree<S: IntoString>(stream: S) -> Result<Rc<RefCell<TreeNode>>, String> {
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

pub mod syntax_tree {
    use super::{generate_relation_tree, print_tree, IntoString, TreeNode};

    use std::{
        cell::{Ref, RefCell},
        collections::HashMap,
        rc::Rc,
    };

    pub struct SyntaxValidationTree {
        entry_points: HashMap<&'static str, Rc<RefCell<TreeNode>>>,
    }

    #[allow(dead_code)]
    impl SyntaxValidationTree {
        pub fn from(syntax_streams: Vec<(&'static str, &'static str)>) -> Self {
            let mut syntax_tree = Self {
                entry_points: HashMap::new(),
            };
            for (entry, stream) in syntax_streams {
                syntax_tree.entry_points.insert(
                    entry,
                    generate_relation_tree(format!("{entry} {stream}").as_str()).unwrap(),
                );
            }
            syntax_tree
        }

        pub fn show_entry<'a, S: IntoString>(&self, entry: S) -> Result<(), String> {
            if let Some(tree) = self.entry_points.get(entry.clone().into().as_str()) {
                println!("Showing for entry: {}", entry);
                print_tree(&*tree.borrow(), 0, &mut vec![]);
            } else {
                return Err(format!("Entry {} does not exist", entry));
            }
            Ok(())
        }

        pub(super) fn get_entry(&self, id: impl IntoString) -> Option<Ref<TreeNode>> {
            let res = self.entry_points.get(id.into().as_str())?;
            return Some(res.borrow());
        }

        pub fn entries(&self) -> Vec<String> {
            let mut entrylist = Vec::new();
            for (entry, _) in &self.entry_points {
                entrylist.push(entry.to_string());
            }
            entrylist
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_entry() {
        let tree = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
        assert!(tree.show_entry("test").is_ok());
        assert!(tree.show_entry("fail").is_err());
    }

    #[test]
    fn test_get_entry() {
        let tree = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
        assert!(tree.get_entry("test").is_some());
        assert!(tree.get_entry("fail").is_none());
    }

    #[test]
    fn test_entries() {
        let tree_full = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
        let tree_empty = syntax_tree::SyntaxValidationTree::from(vec![]);
        assert!(!tree_full.entries().is_empty());
        assert!(tree_empty.entries().is_empty());
    }

    #[test]
    fn test_treenode_get() {
        let tree = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
        let node = tree.get_entry("test").unwrap();

        let node1 = node.get("dummy");

        assert!(node1.is_some());
        assert!(node.get("fail").is_none());
        assert!(node1.as_ref().unwrap().get("syntax").is_some());
        assert!(node1.unwrap().get("fail").is_none());
    }
}
