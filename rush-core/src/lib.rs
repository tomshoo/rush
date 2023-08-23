pub mod lazybuf;
pub mod tracker;

/// The [`BufferExt`] trait allows extending lazy iterators to look ahead n times without consuming
/// the inner iterator. It acheives this by maintaining a buffer of items that can be yeilded by an
/// iterator, and flushing the iterator to this buffer as needed.
///
/// Example: ```
/// use lazybuf::LazyBuf;
///
/// let string = String::from("Hello world");
/// let lbuffr = LazyBuf::new(string.chars());

/// assert_eq!(lbuffr.get(5).copied(), string.chars().nth(5));
/// assert_eq!(lbuffr[5], string.chars().nth(5).unwrap());
/// assert_eq!(lbuffr[string.len() - 1], string.chars().last().unwrap());
///
/// lbuffr.zip(string.chars()).for_each(|(c1, c2)| assert_eq!(*c1, c2));
/// ```
pub trait BufferExt<'a, T>
where
    Self: Iterator<Item = T> + 'a,
    Self: Sized,
{
    fn lazy_buf(self) -> lazybuf::LazyBuf<'a, Self>;
    fn lazy_buf_with_base_capacity(self, capacity: usize) -> lazybuf::LazyBuf<'a, Self>;
}

impl<'a, I: Iterator + 'a> BufferExt<'a, <I as Iterator>::Item> for I {
    fn lazy_buf(self) -> lazybuf::LazyBuf<'a, Self> {
        lazybuf::LazyBuf::new(self)
    }

    fn lazy_buf_with_base_capacity(self, capacity: usize) -> lazybuf::LazyBuf<'a, Self> {
        lazybuf::LazyBuf::with_base_capacity(self, capacity)
    }
}
