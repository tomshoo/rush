use derive_builder::*;
use rush_errors::SVTError;
use std::collections::HashMap;
use std::rc::Rc;

use crate::dtype::SMType;

use super::treeir::{IRGenerator, IntermediateRepr, StreamState};

// Tree path relations
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Relation {
    Optional,
    Path,
}

// Node wrapper for the syntax validation tree
#[derive(Debug, Builder, PartialEq, Eq)]
#[builder(pattern = "owned")]
#[builder(name = "NodeBuilder")]
pub(crate) struct TreeNode {
    #[builder(default = "String::new()")]
    value: String,

    #[builder(default = "self::Relation::Path")]
    relation: Relation,

    #[builder(default = "std::collections::HashMap::new()")]
    joint_nodes: HashMap<String, Rc<TreeNode>>,
}

// Print the tree from the given node
// Utilizes an immensly simplified DFS approach
pub(super) fn print_tree(_root_node: &TreeNode, _ident_level: u8, _visited: &mut Vec<TreeNode>) {
    todo!()
}

// Generate the relation tree
#[allow(unused_variables, unused_mut, unused_assignments)]
pub(super) fn generate_relation_tree(ir: &mut IntermediateRepr) -> Result<Rc<TreeNode>, SVTError> {
    let mut node = NodeBuilder::default().build().unwrap();
    println!("START",);
    while let Some(node) = ir.0.pop() {
        println!("{:?}", node);
        match node.0 {
            SMType::Single((stream, state)) => match state {
                StreamState::Fixed => {}
                StreamState::NotFixed => {
                    generate_relation_tree(&mut IRGenerator::generate_ir(stream)?)?;
                }
            },
            SMType::Multiple(vector) => {
                for (stream, state) in vector.iter() {
                    match state {
                        StreamState::Fixed => {}
                        StreamState::NotFixed => {
                            generate_relation_tree(&mut IRGenerator::generate_ir(stream)?)?;
                        }
                    }
                }
            }
        }
    }
    println!("END",);
    Ok(Rc::new(node))
}
