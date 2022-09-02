pub(crate) mod treeir;
pub(crate) mod treenode;

pub mod syntax_tree {
    use super::treeir;
    use super::treeir::IRGenerator;
    use super::treenode;
    use crate::IntoString;
    use treenode::generate_relation_tree;
    use treenode::print_tree;
    use treenode::TreeNode;

    use std::{
        cell::{Ref, RefCell},
        collections::HashMap,
        rc::Rc,
    };

    pub struct SyntaxValidationTree {
        entry_points: HashMap<&'static str, Rc<TreeNode>>,
    }

    #[allow(dead_code)]
    impl SyntaxValidationTree {
        pub fn from(
            syntax_streams: Vec<(&'static str, &'static str)>,
        ) -> Result<Self, rush_errors::SVTError> {
            let mut syntax_tree = Self {
                entry_points: HashMap::new(),
            };
            for (entry, stream) in &syntax_streams {
                syntax_tree.entry_points.insert(
                    entry,
                    generate_relation_tree(&mut IRGenerator::generate_ir(format!(
                        "{} {}",
                        entry, stream
                    ))?)?,
                );
            }
            Ok(syntax_tree)
        }

        pub fn show_entry<'a, S: IntoString>(&self, entry: S) -> Result<(), String> {
            if let Some(tree) = self.entry_points.get(entry.clone().into().as_str()) {
                println!("Showing for entry: {}", entry);
                print_tree(&*tree, 0, &mut vec![]);
            } else {
                return Err(format!("Entry {} does not exist", entry));
            }
            Ok(())
        }

        pub(super) fn get_entry(&self, id: impl IntoString) -> Option<Rc<TreeNode>> {
            let res = self.entry_points.get(id.into().as_str())?;
            return Some(Rc::clone(res));
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_show_entry() {
//         let tree = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
//         assert!(tree.show_entry("test").is_ok());
//         assert!(tree.show_entry("fail").is_err());
//     }
//
//     #[test]
//     fn test_get_entry() {
//         let tree = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
//         assert!(tree.get_entry("test").is_some());
//         assert!(tree.get_entry("fail").is_none());
//     }
//
//     #[test]
//     fn test_entries() {
//         let tree_full = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
//         let tree_empty = syntax_tree::SyntaxValidationTree::from(vec![]);
//         assert!(!tree_full.entries().is_empty());
//         assert!(tree_empty.entries().is_empty());
//     }
//
//     #[test]
//     fn test_treenode_get() {
//         let tree = syntax_tree::SyntaxValidationTree::from(vec![("test", "dummy !syntax")]);
//         let node = tree.get_entry("test").unwrap();
//
//         let node1 = node.get("dummy");
//
//         assert!(node1.is_some());
//         assert!(node.get("fail").is_none());
//         assert!(node1.as_ref().unwrap().get("syntax").is_some());
//         assert!(node1.unwrap().get("fail").is_none());
//     }
// }
