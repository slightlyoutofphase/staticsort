[![Latest Version]][crates.io] ![Rustc Version nightly]

[Latest Version]: https://img.shields.io/crates/v/staticsort.svg
[crates.io]: https://crates.io/crates/staticsort
[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg
[![Build Status](https://travis-ci.com/slightlyoutofphase/staticsort.svg?branch=master)](https://travis-ci.com/slightlyoutofphase/staticsort)

Implements a macro providing a compile-time quicksort function for arrays of any type and length.

Contributions/suggestions/etc. very welcome!

**Minimum supported Rust version:** due to the use of unstable const fn features, this is a nightly-only crate at the moment.

Fully `#![no_std]` compatible by default.

A basic usage example:

```rust
#![feature(const_fn, const_if_match, const_loop)]

use staticsort::staticsort;

static X: [usize; 12] = [1, 6, 2, 5, 3, 4, 7, 12, 8, 11, 9, 10];
static Y: [f64; 12] = [
  1.0, 6.0, 2.0, 5.0, 3.0, 4.0, 7.0, 12.0, 8.0, 11.0, 9.0, 10.0,
];

// The macro takes the following parameters in the order they're listed:
// type to sort, index to start at, index to end at, total array length, and name of existing
// const / static array variable (or directly-passed "anonymous" array).

// Sort all of X:
static XX: [usize; 12] = staticsort!(usize, 0, 11, 12, X);
// Just sort half of Y:
static YY: [f64; 12] = staticsort!(f64, 0, 6, 12, Y);
// Sort all of an array that's the same as X, but passed directly as a parameter:
static ZZ: [usize; 12] = staticsort!(usize, 0, 11, 12, [1, 6, 2, 5, 3, 4, 7, 12, 8, 11, 9, 10]);

fn main() {
  // Prints: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
  println!("XX: {:?}", XX);
  // Prints: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 12.0, 8.0, 11.0, 9.0, 10.0]
  println!("YY: {:?}", YY);
  // Prints: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
  println!("ZZ: {:?}", ZZ);
}
```

**License:**

Licensed under either the <a href="LICENSE-MIT">MIT license</a> or version 2.0 of the <a href="LICENSE-APACHE">Apache License</a>. Your choice as to which!
Any source code contributions will be dual-licensed in the same fashion.
