use std::fmt::Debug;

use rush_errors::IRError;

use super::treenode::Relation;
use crate::dtype::SMType;
use crate::IntoString;

#[derive(Debug, Clone, Copy)]
enum StreamState {
    Fixed,
    NotFixed,
}

type NodeType = SMType<(String, StreamState), (String, StreamState)>;

#[derive(Debug)]
struct IRNode(NodeType, Relation);
pub struct IRGenerator;

struct IntermediateRepr(Vec<IRNode>);

impl Debug for IntermediateRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "START");
        for item in &self.0 {
            let _ = writeln!(f, "{:?}", item);
        }
        writeln!(f, "END")
    }
}

impl IRNode {
    fn new() -> Self {
        Self(
            SMType::Single((String::new(), StreamState::Fixed)),
            Relation::Path,
        )
    }

    fn push(&mut self, ch: char) {
        match &mut self.0 {
            SMType::Single((string, _)) => string.push(ch),
            SMType::Multiple(vec) => {
                if let Some((last, _)) = vec.last_mut() {
                    last.push(ch)
                } else {
                    vec.push((ch.to_string(), StreamState::Fixed))
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        match &self.0 {
            SMType::Single((string, _)) => string.is_empty(),
            SMType::Multiple(vec) => vec.is_empty(),
        }
    }

    fn set_state(&mut self, state: StreamState) {
        if let Some(item) = self.0.get_multiple_mut() {
            item.last_mut().map_or((), |node| node.1 = state)
        } else {
            self.0.get_single_mut().unwrap().1 = state
        }
    }

    fn get_current_state(&self) -> Option<StreamState> {
        if let Some(item) = self.0.get_single() {
            Some(item.1)
        } else {
            Some(self.0.get_multiple()?.last()?.1)
        }
    }
}

impl IntermediateRepr {
    fn new() -> Self {
        Self(vec![])
    }
    fn push(&mut self, ch: char, parencount: &mut i32) -> Result<(), IRError> {
        Ok(if ch == '(' {
            *parencount += 1
        } else if ch == ')' {
            *parencount -= 1
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
            println!("1",);
            if self.0.is_empty() {
                return Err(IRError::UnexpectedOperator(
                    "Operator \'^\' cannont be used at the beginning of the stream".into(),
                ));
            }
            let mut last = self.0.pop().unwrap();
            match last.0 {
                SMType::Single(item) => {
                    last.0 = SMType::Multiple(vec![item, (String::new(), StreamState::Fixed)])
                }
                _ => {
                    last.0
                        .get_multiple_mut()
                        .unwrap()
                        .push((String::new(), StreamState::Fixed));
                }
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

impl IRGenerator {
    pub fn generate_ir(stream: impl IntoString) -> Result<(), IRError> {
        let stream: String = stream.into();
        let mut ir_stack = IntermediateRepr::new();
        let mut parencount = 0;
        for ch in stream.chars() {
            ir_stack.push(ch, &mut parencount)?;
        }
        println!("{:?}", ir_stack);
        Ok(())
    }
}
