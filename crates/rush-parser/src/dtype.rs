use std::fmt::Debug;

pub enum SMType<T, U> {
    Single(T),
    Multiple(Vec<U>),
}

impl<T, U> PartialEq for SMType<T, U>
where
    T: PartialEq,
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if let SMType::Single(var1) = self {
            if let SMType::Single(var2) = other {
                var1 == var2
            } else {
                false
            }
        } else if let SMType::Multiple(var1) = self {
            if let SMType::Multiple(var2) = other {
                var1 == var2
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<T, U> Eq for SMType<T, U>
where
    T: Eq,
    U: Eq,
{
}

impl<T, U> Clone for SMType<T, U>
where
    T: Clone,
    U: Clone,
{
    fn clone(&self) -> Self {
        match self {
            SMType::Single(val) => SMType::Single(val.clone()),
            SMType::Multiple(val) => SMType::Multiple(val.clone()),
        }
    }
}

impl<T, U> Debug for SMType<T, U>
where
    T: Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SMType::Single(x) => write!(f, "Single({:?})", x),
            SMType::Multiple(x) => write!(f, "Multiple({:?})", x),
        }
    }
}

impl<T, U> SMType<T, U> {
    pub fn get_single(&self) -> Option<&T> {
        if let SMType::Single(val) = self {
            Some(val)
        } else {
            None
        }
    }
    pub fn get_single_mut(&mut self) -> Option<&mut T> {
        if let SMType::Single(val) = self {
            Some(val)
        } else {
            None
        }
    }
    pub fn get_multiple(&self) -> Option<&Vec<U>> {
        if let SMType::Multiple(val) = self {
            Some(val)
        } else {
            None
        }
    }
    pub fn get_multiple_mut(&mut self) -> Option<&mut Vec<U>> {
        if let SMType::Multiple(val) = self {
            Some(val)
        } else {
            None
        }
    }
}
