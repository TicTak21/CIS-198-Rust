#![cfg(test)]

use crate::problem1::{dedup, filter, sum};

#[test]
fn test_sum() {
  let arr: Vec<i32> = vec![1, 23];
  assert_eq!(sum(&arr), 24);
}

#[test]
fn test_dedup() {
  let arr: Vec<i32> = vec![1; 3];
  assert_eq!(dedup(&arr), vec![1]);
}

#[test]
fn test_filter() {
  let arr: Vec<i32> = vec![-1, 5, -2, 10];

  fn is_negative(n: i32) -> bool {
    n < 0
  }

  assert_eq!(filter(&arr, &is_negative), [-1, -2]);
}
