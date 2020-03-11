#![no_std]
#![feature(const_fn, const_if_match, const_loop)]

/// This macro takes the following parameters in the order they're listed:
/// type to sort, index to start at, index to end at, total array length, and name of existing
/// const / static array variable (or directly-passed "anonymous" array).
#[macro_export]
macro_rules! staticsort {
  ($type:ty, $low:expr, $high:expr, $len:expr, $values:expr) => {{
    match $len {
      0 => $values,
      _ => {
        #[inline]
        const fn static_sort(
          mut values: [$type; $len],
          mut low: isize,
          mut high: isize,
        ) -> [$type; $len]
        {
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
                values = static_sort(values, low, j);
              }
              low = i;
            } else {
              if i < high {
                values = static_sort(values, i, high)
              }
              high = j;
            }
            if low >= high {
              break;
            }
          }
          values
        }
        static_sort($values, $low, $high)
      }
    }
  };};
}
