/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
  let mut counter = 0;
  for i in slice {
    counter += i;
  }

  return counter;
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
  let mut v: Vec<i32> = Vec::new();

  for i in vs {
    if !v.contains(i) {
      v.push(*i)
    }
  }

  return v;
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
  let mut v: Vec<i32> = Vec::new();

  for i in vs {
    if pred(*i) == true {
      v.push(*i);
    }
  }

  return v;
}
