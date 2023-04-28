bitflags::bitflags! {
#[derive(Debug, Clone, Copy)]
pub(crate) struct StringState: u8 {
    const Comment = 0b1000;
    const DQuote  = 0b0010;
    const SQuote  = 0b0001;
}}
