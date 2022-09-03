use rush_core::errors::IRError;
use std::fmt::Debug;

use super::node::IRNode;
use super::StreamState;
use crate::dtype::SMType;
use crate::parsers::syntree::treenode::Relation;

pub(crate) struct IntermediateRepr(pub Vec<IRNode>);

impl IntermediateRepr {
    pub(super) fn new() -> Self {
        Self(vec![])
    }
    pub(super) fn push(&mut self, ch: char, parencount: &mut i32) -> Result<(), IRError> {
        Ok(if ch == '(' {
            if *parencount > 0 {
                if let None = self.0.last_mut().map(|node| node.push(ch)) {
                    let mut node = IRNode::new();
                    node.push(ch);
                    node.set_state(StreamState::NotFixed);
                    self.0.push(node);
                }
            }
            *parencount += 1
        } else if ch == ')' {
            *parencount -= 1;
            if *parencount > 0 {
                if let None = self.0.last_mut().map(|node| node.push(ch)) {
                    let mut node = IRNode::new();
                    node.push(ch);
                    node.set_state(StreamState::NotFixed);
                    self.0.push(node);
                }
            }
        } else if *parencount > 0 {
            let (mut node, empty) = self.0.last_mut().map_or((None, false), |node| {
                if node.is_empty() {
                    (Some(node), true)
                } else {
                    (Some(node), false)
                }
            });
            if let (Some(node), true) = (&mut node, empty) {
                node.push(ch);
                node.set_state(StreamState::NotFixed)
            } else if let (Some(node), false) = (&mut node, empty) {
                if let Some(StreamState::Fixed) = node.get_current_state() {
                    let mut node = IRNode::new();
                    node.push(ch);
                    node.set_state(StreamState::NotFixed);
                    self.0.push(node);
                } else {
                    node.push(ch);
                }
            } else {
                let mut node = IRNode::new();
                node.set_state(StreamState::NotFixed);
                self.0.push(node);
            }
        } else if ch.is_whitespace() {
            self.0.push(IRNode::new())
        } else if ch == '!' {
            if let Some(node) =
                self.0
                    .last_mut()
                    .map_or(None, |node| if node.is_empty() { Some(node) } else { None })
            {
                node.1 = Relation::Optional
            } else {
                let mut node = IRNode::new();
                node.1 = Relation::Optional;
                self.0.push(node)
            }
        } else if ch == '^' {
            if self.0.is_empty() {
                return Err(IRError::UnexpectedOperator(
                    "Operator \'^\' cannont be used at the beginning of the stream".into(),
                ));
            }
            let mut last = self.0.pop().unwrap();
            let last_is_empty = last.is_empty();
            match last.0 {
                SMType::Single(item) => {
                    last.0 = SMType::Multiple(vec![item]);
                    last.0
                        .get_multiple_mut()
                        .unwrap()
                        .push((String::new(), StreamState::Fixed));
                    dbg!(&last);
                    dbg!(&last_is_empty);
                }
                _ => last.0.get_multiple_mut().map_or((), |vec| {
                    if vec.last().map_or(false, |node| !node.0.is_empty()) {
                        vec.push((String::new(), StreamState::Fixed))
                    }
                }),
            }
            self.0.push(last);
        } else {
            if let Some(node) = self.0.last_mut() {
                node.push(ch)
            } else {
                self.0.push(IRNode(
                    SMType::Single((ch.to_string(), StreamState::Fixed)),
                    Relation::Path,
                ))
            }
        })
    }
}

impl Debug for IntermediateRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "START");
        for item in &self.0 {
            let _ = writeln!(f, "{:?}", item);
        }
        writeln!(f, "END")
    }
}
