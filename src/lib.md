[Crate](https://crates.io/crates/try-iterator) •
[GitHub](https://github.com/rodrigocfd/try-iterator) •
[Docs](https://docs.rs/try-iterator/)

Adds the following fallible methods to iterators:

| New method | Analog to |
| -- | -- |
| `try_all` | [`all`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all) |
| `try_any` | [`any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) |
| `try_position` | [`position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) |
| `try_rposition` | [`rposition`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition) |

## Motivation

This crate was born out of the necessity of a fallible version for the [`Iterator::position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) method. A [StackOverflow question](https://stackoverflow.com/q/78218651/6923555) was asked, but, contrary to the analog [`try_for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each), the solution was rather cumbersome. This resulted in [this issue](https://github.com/rust-lang/libs-team/issues/361) in the Rust repository, and finally this crate.

Until the standard library adds these methods – if ever –, they are available in this crate.

## Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
try-iterator = { version = "0.0.1" }
```

Then import the `prelude` at the top of your source files:

```rust
use try_iterator::prelude::*;
```

The new methods will be automatically present in [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
