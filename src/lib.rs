#![feature(const_fn, const_if_match, const_loop)]

#[macro_export]
macro_rules! staticsort {
  ($type:ty, $low:expr, $high:expr, $len:expr, $values:expr) => {{
    #[inline(always)]
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
          while p > values[i as usize] {
            i += 1;
          }
          while p < values[j as usize] {
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
  };};
}
