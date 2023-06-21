use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Literal {
    Char(char),
    String(Rc<str>),
    Number(isize),
    Float(f64),
    Boolean(bool),
    Nil,
}
