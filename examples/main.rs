use staticsort::staticsort;

const X: [usize; 12] = [1, 6, 2, 5, 3, 4, 7, 12, 8, 11, 9, 10];

const Y: [f64; 12] = [
  1.0, 6.0, 2.0, 5.0,
  3.0, 4.0, 7.0, 12.0,
  8.0, 11.0, 9.0, 10.0,
];

// The macro takes the following parameters in the order they're
// listed: type to sort, index to start at, index to end at, and
// either the name of an existing const array variable or just
// a directly-passed "anonymous" array.

// Sort all of X:
static XX: [usize; 12] = staticsort!(usize, 0, 11, X);
// Just sort half of Y:
static YY: [f64; 12] = staticsort!(f64, 0, 6, Y);
// Sort all of an array that's the same as X, but passed
// directly as a parameter:
static ZZ: [usize; 12] = staticsort!(
  usize,
  0,
  11,
  [1, 6, 2, 5, 3, 4, 7, 12, 8, 11, 9, 10]
);

/// This is wildly unsound and will only work on ASCII strings
/// in the same (lower/upper)case and numbers.
static STRS: [&'static str; 4] = staticsort!(
  &'static str, 0, 3,
  ["please", "order", "me", "zzz"]
);

fn main() {
  // Prints: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
  println!("XX: {:?}", XX);
  // Prints: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 12.0, 8.0, 11.0, 9.0, 10.0]
  println!("YY: {:?}", YY);
  // Prints: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
  println!("ZZ: {:?}", ZZ);
  // Prints: ["me", "order", "please", "zzz"]
  println!("STRS: {:?}", STRS);

  // Prints: (key, value) pairs: ([32, 42, 69, 82], ["monica", "elon", "mark", "andrzej"])
  const AGES: [u8; 4] = [69, 42, 32, 82];
  let names = ["mark", "elon", "monica", "andrzej"];
  println!(
    "(key, value) pairs: {:?}",
    staticsort!(u8, 0, 3, AGES, names)
  );

  const SIZE: usize = 4;
  let but_not_const_array: [u8; SIZE] = [3,1,2,0];
  // but_not_const_array: [0, 1, 2, 3]
  println!("but_not_const_array: {:?}", staticsort!(u8, 0, 3, but_not_const_array, len SIZE));
}
