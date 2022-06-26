use lazy_static::lazy_static;
use rush_utils::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[macro_use]
extern crate derive_builder;

#[derive(Debug)]
enum Relation {
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

#[allow(dead_code)]
#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
#[builder(name = "NodeBuilder")]
struct TreeNode {
    value: String,
    relation: Relation,

    #[builder(setter(custom))]
    #[builder(default = "HashMap::new()")]
    joint_nodes: HashMap<String, Option<Rc<RefCell<TreeNode>>>>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn insert(&mut self, id: String, node: Rc<RefCell<TreeNode>>) {
        self.joint_nodes.insert(id, Some(node));
    }
    pub fn relation(&mut self, relation: Relation) {
        self.relation = relation;
    }
}

fn get_node_from_value(value: String, relation: Relation) -> Result<Rc<RefCell<TreeNode>>, String> {
    let mut value = value.clone();
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

#[allow(dead_code)]
impl NodeBuilder {
    fn joint_nodes(self, nodes: Vec<(String, TreeNode)>) -> Self {
        let mut map = HashMap::new();
        for (id, node) in nodes {
            map.insert(id.clone(), Some(Rc::from(RefCell::from(node))));
        }
        let mut new = self;
        new.joint_nodes = Some(map);
        new
    }
}

fn print_tree(root_node: &TreeNode, ident_level: u8) {
    let mut ident_string = String::new();
    for _ in 0..ident_level {
        ident_string.push(' ');
    }
    let ident_level = ident_level + 4;
    println!(
        "{}{} {:?}",
        ident_string, root_node.value, root_node.relation
    );
    for (_, node) in &root_node.joint_nodes {
        if let Some(val) = node {
            print_tree(&*val.borrow(), ident_level);
        }
    }
}

#[allow(unused_variables, unused_mut, unused_assignments)]
fn generate_relation_tree(stream: &str) -> Result<Rc<RefCell<TreeNode>>, String> {
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
        let mut dump_buffer = String::new();
        let mut operation_stack: Vec<String> = Vec::new();
        let mut bcount = 0;
        for ch in joint_stream.chars() {
            if ch == '(' {
                if bcount > 0 {
                    dump_buffer.push(ch);
                }
                bcount += 1;
                continue;
            }
            if bcount > 0 {
                if ch == ')' {
                    bcount -= 1
                }
                if bcount > 0 {
                    dump_buffer.push(ch);
                } else {
                    dump_buffer.push_str("+r");
                }
                continue;
            }
            if let Some(relation) = OPERATOR_MAP.get(&ch) {
                match relation {
                    Relation::Optional => optional = true,
                    Relation::Reset => reset = true,
                    Relation::SingleChoice => {
                        if dump_buffer.is_empty() {
                            return Err("No first operand found for operator ^".into());
                        }
                        operation_stack.push(dump_buffer.clone());
                        dump_buffer.clear();
                        oneselect = true;
                    }
                    _ => {}
                }
            } else if ch == ' ' || ch == '\t' {
                if !dump_buffer.is_empty() {
                    // if dump_buffer.ends_with("+r") {
                    //     dump_buffer
                    //         .replace_range((dump_buffer.len() - 2)..(dump_buffer.len() - 1), "");
                    //     let node = generate_relation_tree(&dump_buffer)?;
                    //     if oneselect {
                    //         node.borrow_mut().relation = Relation::SingleChoice;
                    //     }
                    // }
                    if oneselect {
                        operation_stack.push(dump_buffer.clone());
                        for operand in &operation_stack {
                            // let node = NodeBuilder::default()
                            //     .value(operand.clone())
                            //     .relation(Relation::SingleChoice)
                            //     .build()
                            //     .unwrap();
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
                            .insert(dump_buffer.clone(), Rc::clone(&tree_root));
                        reset = false;
                    } else if optional {
                        // current_node.borrow_mut().insert(
                        //     dump_buffer.clone(),
                        //     Rc::from(RefCell::from(
                        //         NodeBuilder::default()
                        //             .value(dump_buffer.clone())
                        //             .relation(Relation::Optional)
                        //             .build()
                        //             .unwrap(),
                        //     )),
                        // );
                        current_node.borrow_mut().insert(
                            dump_buffer.clone(),
                            get_node_from_value(dump_buffer.clone(), Relation::Optional)?,
                        );
                        optional = false;
                    } else {
                        // let node = Rc::from(RefCell::from(
                        //     NodeBuilder::default()
                        //         .value(dump_buffer.clone())
                        //         .relation(Relation::Path)
                        //         .build()
                        //         .unwrap(),
                        // ));
                        let node = get_node_from_value(dump_buffer.clone(), Relation::Path)?;
                        current_node
                            .borrow_mut()
                            .insert(dump_buffer.clone(), Rc::clone(&node));
                        current_node = node;
                    }
                    dump_buffer.clear();
                }
            } else {
                dump_buffer.push(ch);
            }
        }
        if !dump_buffer.is_empty() {
            current_node.borrow_mut().insert(
                dump_buffer.clone(),
                Rc::from(RefCell::from(
                    NodeBuilder::default()
                        .value(dump_buffer.clone())
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
    use super::Token;
    use rush_utils::lexer;

    pub fn analyze(stream: &str) -> Result<Vec<Token>, String> {
        let tree_root = &*super::generate_relation_tree(
            " let !dyn !mut Token !(:: !(nice) DataType) = ($ Token)^Data^Expression ",
        )?;
        super::print_tree(&*tree_root.borrow(), 0);
        lexer::lexer_charwise(&stream)
    }
}
