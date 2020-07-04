/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
  let mut ret_vec = Vec::new();
  let mut bool_vec = vec![false, false];
  for _ in 2..n {
    bool_vec.push(true);
  }

  for i in 0..bool_vec.len() as u32 {
    if bool_vec[i as usize] {
      ret_vec.push(i);
      let mut j = i * i;
      while j < n {
        bool_vec[j as usize] = false;
        j += i;
      }
    }
  }
  ret_vec
}
