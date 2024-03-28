/// Implements the following additional iterator methods:
///
/// * `try_all`;
/// * `try_any`;
/// * `try_position`;
/// * `try_rposition`.
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
