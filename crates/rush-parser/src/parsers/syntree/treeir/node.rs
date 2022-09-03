use super::StreamState;
use crate::dtype::SMType;
use crate::parsers::syntree::treenode::Relation;

type NodeType = SMType<(String, StreamState), (String, StreamState)>;

#[derive(Debug)]
pub(crate) struct IRNode(pub NodeType, pub Relation);

impl IRNode {
    pub(super) fn new() -> Self {
        Self(
            SMType::Single((String::new(), StreamState::Fixed)),
            Relation::Path,
        )
    }

    pub(super) fn push(&mut self, ch: char) {
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

    pub fn is_empty(&self) -> bool {
        match &self.0 {
            SMType::Single((string, _)) => string.is_empty(),
            SMType::Multiple(vec) => vec.is_empty(),
        }
    }

    pub(super) fn set_state(&mut self, state: StreamState) {
        if let Some(item) = self.0.get_multiple_mut() {
            item.last_mut().map_or((), |node| node.1 = state)
        } else {
            self.0.get_single_mut().unwrap().1 = state
        }
    }

    pub fn get_current_state(&self) -> Option<StreamState> {
        if let Some(item) = self.0.get_single() {
            Some(item.1)
        } else {
            Some(self.0.get_multiple()?.last()?.1)
        }
    }
}
