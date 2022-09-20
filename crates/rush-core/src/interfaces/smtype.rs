use std::fmt::Debug;

pub enum SMulType<T, U> {
    Single(T),
    Multiple(Vec<U>),
}

pub type SMType<T> = SMulType<T, T>;

impl<T, U> PartialEq for SMulType<T, U>
where
    T: PartialEq,
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if let SMulType::Single(var1) = self {
            if let SMulType::Single(var2) = other {
                var1 == var2
            } else {
                false
            }
        } else if let SMulType::Multiple(var1) = self {
            if let SMulType::Multiple(var2) = other {
                var1 == var2
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<T, U> Eq for SMulType<T, U>
where
    T: Eq,
    U: Eq,
{
}

impl<T, U> Clone for SMulType<T, U>
where
    T: Clone,
    U: Clone,
{
    fn clone(&self) -> Self {
        match self {
            SMulType::Single(val) => SMulType::Single(val.clone()),
            SMulType::Multiple(val) => SMulType::Multiple(val.clone()),
        }
    }
}

impl<T, U> Debug for SMulType<T, U>
where
    T: Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SMulType::Single(x) => write!(f, "Single({:?})", x),
            SMulType::Multiple(x) => write!(f, "Multiple({:?})", x),
        }
    }
}

impl<T, U> SMulType<T, U> {
    pub fn get_single(&self) -> Option<&T> {
        if let SMulType::Single(val) = self {
            Some(val)
        } else {
            None
        }
    }
    pub fn get_single_mut(&mut self) -> Option<&mut T> {
        if let SMulType::Single(val) = self {
            Some(val)
        } else {
            None
        }
    }
    pub fn get_multiple(&self) -> Option<&Vec<U>> {
        if let SMulType::Multiple(val) = self {
            Some(val)
        } else {
            None
        }
    }
    pub fn get_multiple_mut(&mut self) -> Option<&mut Vec<U>> {
        if let SMulType::Multiple(val) = self {
            Some(val)
        } else {
            None
        }
    }

    /// Returns `true` if the smul type is [`Multiple`].
    ///
    /// [`Multiple`]: SMulType::Multiple
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        matches!(self, Self::Multiple(..))
    }

    /// Returns `true` if the smul type is [`Single`].
    ///
    /// [`Single`]: SMulType::Single
    #[must_use]
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(..))
    }
}
