pub(crate) mod irepr;
pub(crate) mod node;

use rush_core::errors::IRError;

use crate::IntoString;
use irepr::IntermediateRepr;

#[derive(Debug, Clone, Copy)]
pub(crate) enum StreamState {
    Fixed,
    NotFixed,
}

pub struct IRGenerator;

impl IRGenerator {
    pub(crate) fn generate_ir(stream: impl IntoString) -> Result<IntermediateRepr, IRError> {
        let stream: String = stream.into();
        let mut ir_stack = IntermediateRepr::new();
        let mut parencount = 0;
        for ch in stream.chars() {
            ir_stack.push(ch, &mut parencount)?;
        }
        Ok(ir_stack)
    }
}
