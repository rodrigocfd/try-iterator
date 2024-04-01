# TryIterator

[![Crates.io](https://img.shields.io/crates/v/try-iterator.svg)](https://crates.io/crates/try-iterator)
[![Docs.rs](https://docs.rs/try-iterator/badge.svg)](https://docs.rs/try-iterator)
[![Lines of code](https://tokei.rs/b1/github/rodrigocfd/try-iterator)](https://github.com/rodrigocfd/try-iterator)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Implements the [`TryIterator`](https://docs.rs/try-iterator/latest/try_iterator/prelude/trait.TryIterator.html) trait, which will add the following fallible methods to [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html):

| New method | Analog to |
| -- | -- |
| [`try_all`](https://docs.rs/try-iterator/latest/try_iterator/prelude/trait.TryIterator.html#method.try_all) | [`all`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all) |
| [`try_any`](https://docs.rs/try-iterator/latest/try_iterator/prelude/trait.TryIterator.html#method.try_any) | [`any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) |
| [`try_position`](https://docs.rs/try-iterator/latest/try_iterator/prelude/trait.TryIterator.html#method.try_position) | [`position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) |
| [`try_rposition`](https://docs.rs/try-iterator/latest/try_iterator/prelude/trait.TryIterator.html#method.try_rposition) | [`rposition`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition) |

## Motivation

This crate was born out of necessity of a fallible version for the [`Iterator::position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) method, which was asked in [this StackOverflow question](https://stackoverflow.com/q/78218651/6923555). Contrary to the equivalent [`try_for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each), the solution is rather cumbersome, which resulted in [this issue](https://github.com/rust-lang/libs-team/issues/361) in the Rust repository.

Until the standard library adds these fallible methods – if ever –, they are available in this crate.

## Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
try-iterator = { version = "1.0.0" }
```

Then import the `prelude` at the top of your source files:

```rust
use try_iterator::prelude::*;
```

The new methods will be automatically present in [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
