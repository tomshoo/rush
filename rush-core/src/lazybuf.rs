#![cfg_attr(debug_assertions, allow(dead_code, unused))]

use std::cell::RefCell;
use std::cell::UnsafeCell;

pub struct LazyBuf<'l, I: Iterator> {
    iter: RefCell<I>,
    nread: usize,
    ibuf: UnsafeCell<Vec<I::Item>>,

    _life: std::marker::PhantomData<&'l I::Item>,
}

impl<I: Iterator> LazyBuf<'_, I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter: RefCell::new(iter),
            nread: 0,
            ibuf: UnsafeCell::new(vec![]),

            _life: Default::default(),
        }
    }

    pub(crate) fn with_base_capacity(iter: I, capacity: usize) -> Self {
        Self {
            iter: RefCell::new(iter),
            nread: 0,
            ibuf: UnsafeCell::new(Vec::with_capacity(capacity)),

            _life: Default::default(),
        }
    }
}

impl<T: Iterator> std::ops::Index<usize> for LazyBuf<'_, T> {
    type Output = T::Item;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.get(index) }.unwrap_or_else(|| panic!("index out of bounds {}", index))
    }
}

impl<'a, T: Iterator + 'a> Iterator for LazyBuf<'a, T> {
    type Item = &'a T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let item = unsafe { &*self.ibuf.get() }.get(self.nread).or_else(|| {
            let temp = self.iter.get_mut().next()?;
            self.ibuf.get_mut().push(temp);

            unsafe { &*self.ibuf.get() }.get(self.nread)
        })?;

        self.nread += 1;
        Some(item)
    }
}

impl<'a, T: Iterator + 'a> LazyBuf<'a, T> {
    /// Similar to [`Iterator::nth`] except does not consume the iterator, i.e, does not modify its
    /// state.
    ///
    /// # Example
    /// ```
    /// use rush_core::lazybuf::LazyBuf;
    /// use rush_core::BufferExt;
    ///
    /// let     string = String::from("asd");
    /// let mut buffer = string.chars().lazy_buf();
    ///
    /// assert_eq!(buffer.get(1).copied(), Some('s'));
    /// assert_eq!(buffer.next().copied(), Some('a'));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T::Item> {
        let index_ref = unsafe { &*self.ibuf.get() }.get(index);
        index_ref.or_else(|| unsafe {
            let take_amt = index - { &*self.ibuf.get() }.len() + 1;
            let ibuff_extension = {
                let mut extension = Vec::with_capacity(take_amt);
                let mut itermut = self.iter.borrow_mut();

                itermut
                    .by_ref()
                    .take(take_amt)
                    .for_each(|t| extension.push(t));

                extension
            };

            let new_ref = &mut *self.ibuf.get();
            new_ref.extend(ibuff_extension.into_iter());

            new_ref.get(index)
        })
    }

    /// Get the next item in the iterator without modifying the current state, i.e, [`LazyBuf`] is
    /// not consumed.
    ///
    /// # Example
    /// ```
    /// use rush_core::lazybuf::LazyBuf;
    /// use rush_core::BufferExt;
    ///
    /// let     string = String::from("Hello");
    /// let mut buffer = string.chars().lazy_buf();
    ///
    /// assert_eq!(buffer.next().copied(), Some('H'));
    /// assert_eq!(buffer.peek().copied(), Some('e'));
    /// assert_eq!(buffer.next().copied(), Some('e'));
    /// ```
    pub fn peek(&self) -> Option<&T::Item> {
        self.get(self.nread)
    }

    /// Consumes the iterator returing a vector of items contained within.
    ///
    /// # Example
    /// ```
    /// use rush_core::lazybuf::LazyBuf;
    /// use rush_core::BufferExt;
    ///
    /// let     string = String::from("asd");
    /// let mut lbuffr = string.chars().lazy_buf();
    ///
    /// let vector = lbuffr.consume();
    ///
    /// vector.into_iter().zip(string.chars()).for_each(|(c1, c2)| assert_eq!(c1, c2));
    /// ```
    pub fn consume(mut self) -> Vec<T::Item> {
        while self.next().is_some() {}

        let Self { ibuf, .. } = self;
        ibuf.into_inner()
    }

    /// Takes the iterator and moves the cursor to the given point returning a new iterator, moves
    /// the buffer to prevent state mixing.
    ///
    /// Panics if the give point is greater than the iterator length.
    ///
    /// # Example
    /// ```
    /// use rush_core::lazybuf::LazyBuf;
    /// use rush_core::BufferExt;
    ///
    /// let     string = String::from("asd");
    /// let mut buffer = string.chars().lazy_buf();
    ///
    /// assert_eq!(buffer.next().copied(), Some('a'));
    /// assert_eq!(buffer.next().copied(), Some('s'));
    ///
    /// let mut buffer = buffer.reset(0);
    /// assert_eq!(buffer.next().copied(), Some('a'))
    /// ```
    pub fn reset(mut self, point: usize) -> Self {
        if point >= self.len() {
            panic!("point index out of bounds: {point}");
        }
        self.nread = point;
        self
    }

    /// Returns the length of the complete iterator by consuming the inner iterator, does not
    /// modify the current state of [`LazyBuf`].
    ///
    /// # Example
    /// ```
    /// use rush_core::lazybuf::LazyBuf;
    /// use rush_core::BufferExt;
    ///
    /// let     string = String::from("asd");
    /// let mut buffer = string.chars().lazy_buf();
    ///
    /// assert_eq!(buffer.len(), string.len());
    /// assert_eq!(buffer.next().copied(), Some('a'));
    /// ```
    pub fn len(&mut self) -> usize {
        let nread = self.nread;
        while self.next().is_some() {}
        self.nread = nread;

        self.ibuf.get_mut().len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let string = String::from("Hello world");
        let chars = string.chars();

        let lbuf = LazyBuf::new(string.chars());
        lbuf.zip(chars).for_each(|(c1, c2)| assert_eq!(*c1, c2));
    }

    #[test]
    fn test_index() {
        let string = String::from("Hello world");
        let lbuffr = LazyBuf::new(string.chars());

        assert_eq!(lbuffr.get(5).copied(), string.chars().nth(5));
        assert_eq!(lbuffr[5], string.chars().nth(5).unwrap());
        assert_eq!(lbuffr[string.len() - 1], string.chars().last().unwrap());
    }

    #[test]
    fn test_both() {
        let string = String::from("Hello world");
        let lbuffr = LazyBuf::new(string.chars());

        assert_eq!(lbuffr[string.len() - 1], string.chars().last().unwrap());

        lbuffr
            .zip(string.chars())
            .for_each(|(c1, c2)| assert_eq!(*c1, c2));
    }

    #[test]
    fn test_out_of_bounds() {
        let string = String::from("Hello world");
        let lbuffr = LazyBuf::new(string.chars());

        assert_eq!(lbuffr.get(string.len()), None);
    }
}
