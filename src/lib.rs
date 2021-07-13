#![no_std]
#![allow(incomplete_features)]
#![allow(unused_variables)]
#![feature(const_fn_floating_point_arithmetic, const_generics, const_fn_trait_bound)]

#[doc(hidden)]
pub struct __StaticSorter<T: Copy + PartialOrd, const N: usize> {
  marker: core::marker::PhantomData<T>,
}

macro_rules! impl_quick_sort {
  ($name:ident, $type:ty, $values:ident, $compared:ident, $i:ident, $j:ident, $p:ident, $cmp:block $(,$add_swap:block)? $(,arg $extra_name:ident: $extra_type:ty),* $(,gen $gen_name:ident : $gen_type:path),*) => {
    impl<const N: usize> __StaticSorter<$type, N> {
      #[inline]
    pub const fn $name<$($gen_name: $gen_type,)*>(
      mut $values: [$type; N], $(mut $extra_name: $extra_type,)* mut low: isize, mut high: isize
    ) -> ([$type; N], $($extra_type),*) {
      let range = high - low;
      if range <= 0 || range >= $values.len() as isize {
        return ($values, $($extra_name),*);
      }
      loop {
        let mut $i = low;
        let mut $j = high;
        let $p = $values[(low + ((high - low) >> 1)) as usize];
        loop {
          let mut $compared = $i;
          while let Ordering::Smaller = $cmp {
            $compared += 1;
          }
          $i = $compared;
          let mut $compared = $j;
          while let Ordering::Greater = $cmp {
            $compared -= 1;
          }
          $j = $compared;
          if $i <= $j {
            if $i != $j {
              let q = $values[$i as usize];
              $values[$i as usize] = $values[$j as usize];
              $values[$j as usize] = q;

              $($add_swap;)?
            }
            $i += 1;
            $j -= 1;
          }
          if $i > $j {
            break;
          }
        }
        if $j - low < high - $i {
          if low < $j {
            let ($values, $($extra_name),*) = Self::$name::<$($gen_name,)*>($values, $($extra_name,)* low, $j);
          }
          low = $i;
        } else {
          if $i < high {
            let ($values, $($extra_name),*) = Self::$name::<$($gen_name,)*>($values, $($extra_name,)* $i, high);
          }
          high = $j;
        }
        if low >= high {
          break;
        }
      }
      ($values, $($extra_name),*)
    }
  }
};}

/// This is a hack to work around fully generic (and non-primitive in general)
/// types not being comparable (PartialOrd's `.cmp()` not being const) in `const fn` contexts yet.
/// Once they are, the macro impls for specific types will be removed.
#[doc(hidden)]
macro_rules! impl_static_sorter {
  ($type:ty) => {
    impl_quick_sort!(
      __static_sort, $type, values, compared,i,j,p,
      {
        if values[compared as usize] < p {
          Ordering::Smaller
        } else if values[compared as usize] == p {
          Ordering::Equal
        } else {
          Ordering::Greater
        }
    });

    impl_quick_sort!(
      __static_co_sort, $type,
      values,compared,i,j,p,
      {
        if values[compared as usize] < p {
          Ordering::Smaller
        } else if values[compared as usize] == p {
          Ordering::Equal
        } else {
          Ordering::Greater
        }
      },
      {
        let q = keys[i as usize];
        keys[i as usize] = keys[j as usize];
        keys[j as usize] = q;
      },
      arg keys: [K; N],
      gen K: ::core::marker::Copy
    );
  }
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

impl_quick_sort!(
  __static_sort, &'static str,
  values,compared, i, j, p,
  {str_ord(values[compared as usize], p)}
);

impl_quick_sort!(
  __static_co_sort, &'static str,
  values,compared,i,j,p,
  {
    str_ord(values[compared as usize], p)
  },
  {
    let q = keys[i as usize];
    keys[i as usize] = keys[j as usize];
    keys[j as usize] = q;
  },
  arg keys: [K; N],
  gen K: ::core::marker::Copy);

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
  let a_bytes_len = a_bytes.len();
  let b_bytes = b.as_bytes();
  let b_bytes_len = b_bytes.len();
  let len = if a_bytes_len > b_bytes_len { b_bytes_len } else { a_bytes_len };
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
  return if a_bytes_len == b_bytes_len {
    Ordering::Equal
  } else if a_bytes_len > b_bytes_len {
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

/// This macro takes the following parameters in the order they're listed: type to sort, index to
/// start at, index to end at, and either the name of an existing `const` array variable or just a
/// a directly-passed "anonymous" array.
#[macro_export]
macro_rules! staticsort {
  ($type:ty, $low:expr, $high:expr, $values:expr) => {{
    const LEN: usize = $values.len();
    $crate::__StaticSorter::<$type, LEN>::__static_sort($values, $low, $high).0
  };};
  ($type:ty, $low:expr, $high:expr, $values:expr, $keys:expr) => {{
    const LEN: usize = $values.len();
    $crate::__StaticSorter::<$type, LEN>::__static_co_sort($values, $keys, $low, $high)
  };};
}
