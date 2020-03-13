#![no_std]
#![allow(incomplete_features)]
#![feature(const_fn, const_generics, const_if_match, const_loop)]

#[doc(hidden)]
pub struct __StaticSorter<'a, T: 'a + Copy + PartialOrd, const N: usize> {
  marker: core::marker::PhantomData<&'a T>,
}

/// This is a hack to work around fully generic (and non-primitive in general)
/// types not being comparable in `const fn` contexts yet. Once they are, the
/// macro impls for specific types will be removed.
#[doc(hidden)]
macro_rules! impl_static_sorter {
  ($type:ty) => {
    impl<const N: usize> __StaticSorter<'_, $type, N> {
      #[inline]
      pub const fn __static_sort(
        mut values: [$type; N],
        mut low: isize,
        mut high: isize,
      ) -> [$type; N] {
        if high - low <= 0 {
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

/// This macro takes the following parameters in the order they're listed: type to sort, index to
/// start at, index to end at, and either the name of an existing `const` array variable or just a
/// a directly-passed "anonymous" array.
#[macro_export]
macro_rules! staticsort {
  ($type:ty, $low:expr, $high:expr, $values:expr) => {{
    const LEN: usize = $values.len();
    match LEN {
      0 => $values,
      _ => $crate::__StaticSorter::<$type, LEN>::static_sort($values, $low, $high),
    }
  };};
}
