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
