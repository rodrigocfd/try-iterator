/// Implements the following fallible iterator methods:
///
/// * `try_all`;
/// * `try_any`;
/// * `try_position`;
/// * `try_rposition`.
///
/// Prefer importing this trait through the crate prelude:
///
/// ```
/// use try_iterator::prelude::*;
/// ```
pub trait TryIterator: Iterator {
	/// Tests if every element of the iterator matches a predicate, stopping at
	/// the first error and returning that error.
	///
	/// This can also be thought of as the fallible form of
	/// [`all()`](Iterator::all).
	///
	/// # Examples
	///
	/// Ordinary operation:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Ok("foo"),
	///     Ok("foo"),
	/// ];
	///
	/// let res = items.iter()
	///     .try_all(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "foo";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(res.is_ok(), true);
	/// assert_eq!(res.unwrap(), true);
	/// ```
	///
	/// Fails the whole operation when an [`Err`] is present:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Err(4444),
	///     Ok("foo"),
	/// ];
	///
	/// let res = items.iter()
	///     .try_all(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "foo";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(res.is_err(), true);
	/// ```
	fn try_all<E, F>(&mut self, mut predicate: F) -> Result<bool, E>
		where Self: Sized,
			F: FnMut(Self::Item) -> Result<bool, E>,
	{
		for item in self {
			if !predicate(item)? {
				return Ok(false)
			}
		}
		Ok(true)
	}

	/// Tests if any element of the iterator matches a predicate, stopping at
	/// the first error and returning that error.
	///
	/// This can also be thought of as the fallible form of
	/// [`any()`](Iterator::any).
	///
	/// # Examples
	///
	/// Ordinary operation:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Ok("ayy"),
	///     Ok("bar"),
	/// ];
	///
	/// let res = items.iter()
	///     .try_any(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "bar";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(res.is_ok(), true);
	/// assert_eq!(res.unwrap(), true);
	/// ```
	///
	/// Fails the whole operation when an [`Err`] is present:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Err(7777),
	///     Ok("bar"),
	/// ];
	///
	/// let res = items.iter()
	///     .try_any(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "bar";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(res.is_err(), true);
	/// ```
	fn try_any<E, F>(&mut self, mut predicate: F) -> Result<bool, E>
		where Self: Sized,
			F: FnMut(Self::Item) -> Result<bool, E>,
	{
		for item in self {
			if predicate(item)? {
				return Ok(true)
			}
		}
		Ok(false)
	}

	/// Searches for an element in an iterator, returning its index, stopping at
	/// the first error and returning that error.
	///
	/// This can also be thought of as the fallible form of
	/// [`position()`](Iterator::position).
	///
	/// # Examples
	///
	/// Ordinary operation:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Ok("ayy"),
	///     Ok("bar"),
	/// ];
	///
	/// let pos = items.iter()
	///     .try_position(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "bar";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(pos.is_ok(), true);
	/// assert_eq!(pos.unwrap(), Some(2));
	/// ```
	///
	/// Fails the whole operation when an [`Err`] is present:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Err(8888),
	///     Ok("bar"),
	/// ];
	///
	/// let pos = items.iter()
	///     .try_any(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "bar";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(pos.is_err(), true);
	/// ```
	fn try_position<E, F>(&mut self, mut predicate: F) -> Result<Option<usize>, E>
		where Self: Sized,
			F: FnMut(Self::Item) -> Result<bool, E>,
	{
		for (idx, item) in self.enumerate() {
			if predicate(item)? {
				return Ok(Some(idx));
			}
		}
		Ok(None)
	}

	/// Searches for an element in an iterator from the right, returning its
	/// index, stopping at the first error and returning that error.
	///
	/// This can also be thought of as the fallible form of
	/// [`rposition()`](Iterator::rposition).
	///
	/// # Examples
	///
	/// Ordinary operation:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Ok("ayy"),
	///     Ok("bar"),
	/// ];
	///
	/// let pos = items.iter()
	///     .try_rposition(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "foo";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(pos.is_ok(), true);
	/// assert_eq!(pos.unwrap(), Some(0));
	/// ```
	///
	/// Fails the whole operation when an [`Err`] is present:
	///
	/// ```
	/// use try_iterator::prelude::*;
	///
	/// let items: &[Result<&str, u32>] = &[
	///     Ok("foo"),
	///     Err(9999),
	///     Ok("bar"),
	/// ];
	///
	/// let pos = items.iter()
	///     .try_rposition(|item| -> Result<_, u32> {
	///         let equal = (*item)? == "foo";
	///         Ok(equal)
	///     });
	///
	/// assert_eq!(pos.is_err(), true);
	/// ```
	fn try_rposition<E, F>(&mut self, mut predicate: F) -> Result<Option<usize>, E>
		where Self: Sized + ExactSizeIterator + DoubleEndedIterator,
			F: FnMut(Self::Item) -> Result<bool, E>,
	{
		for (idx, item) in self.enumerate().rev() {
			if predicate(item)? {
				return Ok(Some(idx));
			}
		}
		Ok(None)
	}
}

impl<'a, T> TryIterator for core::slice::Iter<'a, T> {}
impl<A, B> TryIterator for std::iter::Chain<A, B> where A: Iterator, B: Iterator<Item = <A as Iterator>::Item> {}
impl<'a, I, T> TryIterator for std::iter::Cloned<I> where T: 'a + Clone, I: Iterator<Item = &'a T> {}
impl<'a, I, T> TryIterator for std::iter::Copied<I> where T: 'a + Copy, I: Iterator<Item = &'a T> {}
impl<I> TryIterator for std::iter::Cycle<I> where I: Clone + Iterator {}
impl<T> TryIterator for std::iter::Empty<T> {}
impl<I> TryIterator for std::iter::Enumerate<I> where I: Iterator {}
impl<I, P> TryIterator for std::iter::Filter<I, P> where I: Iterator, P: FnMut(&<I as Iterator>::Item) -> bool {}
impl<B, I, F> TryIterator for std::iter::FilterMap<I, F> where I: Iterator, F: FnMut(<I as Iterator>::Item) -> Option<B> {}
impl<I, U, F> TryIterator for std::iter::FlatMap<I, U, F> where I: Iterator, U: IntoIterator, F: FnMut(<I as Iterator>::Item) -> U {}
impl<I, U> TryIterator for std::iter::Flatten<I> where I: Iterator, <I as Iterator>::Item: IntoIterator<IntoIter = U, Item = <U as Iterator>::Item>, U: Iterator {}
impl<T, F> TryIterator for std::iter::FromFn<F> where F: FnMut() -> Option<T> {}
impl<I> TryIterator for std::iter::Fuse<I> where I: Iterator {}
impl<I, F> TryIterator for std::iter::Inspect<I, F> where I: Iterator, F: FnMut(&<I as Iterator>::Item) {}
impl<B, I, F> TryIterator for std::iter::Map<I, F> where I: Iterator, F: FnMut(<I as Iterator>::Item) -> B {}
impl<B, I, P> TryIterator for std::iter::MapWhile<I, P> where I: Iterator, P: FnMut(<I as Iterator>::Item) -> Option<B> {}
impl<T> TryIterator for std::iter::Once<T> {}
impl<A, F> TryIterator for std::iter::OnceWith<F> where F: FnOnce() -> A {}
impl<I> TryIterator for std::iter::Peekable<I> where I: Iterator {}
impl<A> TryIterator for std::iter::Repeat<A> where A: Clone {}
impl<A, F> TryIterator for std::iter::RepeatWith<F> where F: FnMut() -> A {}
impl<I> TryIterator for std::iter::Rev<I> where I: DoubleEndedIterator {}
impl<B, I, St, F> TryIterator for std::iter::Scan<I, St, F> where I: Iterator, F: FnMut(&mut St, <I as Iterator>::Item) -> Option<B> {}
impl<I> TryIterator for std::iter::Skip<I> where I: Iterator {}
impl<I, P> TryIterator for std::iter::SkipWhile<I, P> where I: Iterator, P: FnMut(&<I as Iterator>::Item) -> bool {}
impl<I> TryIterator for std::iter::StepBy<I> where I: Iterator {}
impl<T, F> TryIterator for std::iter::Successors<T, F> where F: FnMut(&T) -> Option<T> {}
impl<I> TryIterator for std::iter::Take<I> where I: Iterator {}
impl<I, P> TryIterator for std::iter::TakeWhile<I, P> where I: Iterator, P: FnMut(&<I as Iterator>::Item) -> bool {}
impl<A, B> TryIterator for std::iter::Zip<A, B> where A: Iterator, B: Iterator {}
