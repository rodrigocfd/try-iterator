#![doc = include_str!("lib.md")]

mod try_iterator;

pub mod prelude {
    //! This prelude is intended to be a glob import to all your modules that
    //! make use the library:
    //!
    //! ```rust,no_run
	//! use try_iterator::prelude::*;
	//! ```

    pub use super::try_iterator::*;
}
