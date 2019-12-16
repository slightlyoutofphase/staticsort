#![feature(const_fn, const_if_match, const_loop)]

#[macro_export]
macro_rules! staticsort {
  ($type:ty, $low:expr, $high:expr, $len:expr, $values:expr) => {{
    #[inline(always)]
    const fn static_sort(values: [$type; $len], mut low: isize, mut high: isize) -> [$type; $len] {
      if high - low <= 0 {
        return values;
      }
      let mut res = values;
      loop {
        let mut i = low;
        let mut j = high;
        let p = res[(low + ((high - low) >> 1)) as usize];
        loop {
          while p > res[i as usize] {
            i += 1;
          }
          while p < res[j as usize] {
            j -= 1;
          }
          if i <= j {
            if i != j {
              let q = res[i as usize];
              res[i as usize] = res[j as usize];
              res[j as usize] = q;
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
            res = static_sort(res, low, j);
          }
          low = i;
        } else {
          if i < high {
            res = static_sort(res, i, high)
          }
          high = j;
        }
        if low >= high {
          break;
        }
      }
      res
    }
    static_sort($values, $low, $high)
  };};
}
