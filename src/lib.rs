#![no_std]
#![allow(incomplete_features)]
#![feature(const_fn_floating_point_arithmetic, const_generics)]

#[doc(hidden)]
pub struct __StaticSorter<T: Copy + PartialOrd, const N: usize> {
  marker: core::marker::PhantomData<T>,
}

/// This is a hack to work around fully generic (and non-primitive in general)
/// types not being comparable in `const fn` contexts yet. Once they are, the
/// macro impls for specific types will be removed.
#[doc(hidden)]
macro_rules! impl_static_sorter {
  ($type:ty) => {
    impl<const N: usize> __StaticSorter<$type, N> {
      #[inline]
      pub const fn __static_sort(
        mut values: [$type; N],
        mut low: isize,
        mut high: isize,
      ) -> [$type; N] {
        let range = high - low;
        if range <= 0 || range >= values.len() as isize {
          return values;
        }
        loop {
          let mut i = low;
          let mut j = high;
          let p = values[(low + ((high - low) >> 1)) as usize];
          loop {
            while values[i as usize] < p {
              i += 1;
            }
            while values[j as usize] > p {
              j -= 1;
            }
            if i <= j {
              if i != j {
                let q = values[i as usize];
                values[i as usize] = values[j as usize];
                values[j as usize] = q;
              }
              i += 1;
              j -= 1;
            }
            if i > j {
              break;
            }
          }
          if j - low < high - i {
            if low < j {
              values = Self::__static_sort(values, low, j);
            }
            low = i;
          } else {
            if i < high {
              values = Self::__static_sort(values, i, high)
            }
            high = j;
          }
          if low >= high {
            break;
          }
        }
        values
      }
    }
  };
}

impl_static_sorter!(bool);
impl_static_sorter!(char);
impl_static_sorter!(u8);
impl_static_sorter!(u16);
impl_static_sorter!(u32);
impl_static_sorter!(u64);
impl_static_sorter!(u128);
impl_static_sorter!(usize);
impl_static_sorter!(i8);
impl_static_sorter!(i16);
impl_static_sorter!(i32);
impl_static_sorter!(i64);
impl_static_sorter!(i128);
impl_static_sorter!(isize);
impl_static_sorter!(f32);
impl_static_sorter!(f64);

enum Ordering {
  /// Yeah it should be called smaller not less imo
  Smaller,
  Equal,
  Greater
}

/// This is wildly unsound and will only work on ASCII strings
/// in the same (lower/upper)case and numbers.
const fn str_ord(a: &'static str, b: &'static str) -> Ordering {
  let a_bytes = a.as_bytes();
  let b_bytes = b.as_bytes();
  let len = if a_bytes.len() > b_bytes.len() { b_bytes.len() } else { a_bytes.len() };
  let mut i = 0;
  loop {
    if i == len {
      break;
    }
    if a_bytes[i] > b_bytes[i] {
      return Ordering::Greater;
    } else if a_bytes[i] < b_bytes[i] {
      return Ordering::Smaller;
    }

    i += 1;
  }
  return if a_bytes.len() == b_bytes.len() {
    Ordering::Equal
  } else if a_bytes.len() > b_bytes.len() {
    Ordering::Greater
  } else {
    Ordering::Smaller
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn ord_eq(a: Ordering, b: Ordering) -> bool {
    match (a,b) {
      (Ordering::Greater, Ordering::Greater) => true,
      (Ordering::Smaller, Ordering::Smaller) => true,
      (Ordering::Equal, Ordering::Equal) => true,
      _ => false
    }
  }

  #[test]
  fn test_str_ord() {
    assert!(ord_eq(str_ord("a", "z"), Ordering::Smaller));

    assert!(ord_eq(str_ord("aa", "aa"), Ordering::Equal));
    assert!(ord_eq(str_ord("a", "aa"), Ordering::Smaller));
    assert!(ord_eq(str_ord("aa", "a"), Ordering::Greater));

    assert!(ord_eq(str_ord("ba", "aa"), Ordering::Greater));
  }
}

impl<const N: usize> __StaticSorter<&'static str, N> {
  #[inline]
  pub const fn __static_sort(
    mut values: [&'static str; N],
    mut low: isize,
    mut high: isize,
  ) -> [&'static str; N] {
    let range = high - low;
    if range <= 0 || range >= values.len() as isize {
      return values;
    }
    loop {
      let mut i = low;
      let mut j = high;
      let p = values[(low + ((high - low) >> 1)) as usize];
      loop {
        while let Ordering::Smaller = str_ord(values[i as usize],p) {
          i += 1;
        }
        while let Ordering::Greater = str_ord(values[j as usize], p) {
          j -= 1;
        }
        if i <= j {
          if i != j {
            let q = values[i as usize];
            values[i as usize] = values[j as usize];
            values[j as usize] = q;
          }
          i += 1;
          j -= 1;
        }
        if i > j {
          break;
        }
      }
      if j - low < high - i {
        if low < j {
          values = Self::__static_sort(values, low, j);
        }
        low = i;
      } else {
        if i < high {
          values = Self::__static_sort(values, i, high)
        }
        high = j;
      }
      if low >= high {
        break;
      }
    }
    values
  }
}


/// This macro takes the following parameters in the order they're listed: type to sort, index to
/// start at, index to end at, and either the name of an existing `const` array variable or just a
/// a directly-passed "anonymous" array.
#[macro_export]
macro_rules! staticsort {
  ($type:ty, $low:expr, $high:expr, $values:expr) => {{
    const LEN: usize = $values.len();
    $crate::__StaticSorter::<$type, LEN>::__static_sort($values, $low, $high)
  };};
}
