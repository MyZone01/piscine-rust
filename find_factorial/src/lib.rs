pub fn factorial(num: u64) -> u64 {
  factorial_fold(num)
}

pub fn factorial_iterative(num: u64) -> u64 {
  let mut result = 1;
  for n in 1..num+1 {
    result *= n;
  }
  result
}

pub fn factorial_recursive_with_if(num: u64) -> u64 {
  if num == 0 {
    1
  } else {
    num * factorial_recursive_with_if(num - 1)
  }
}

pub fn factorial_recursive_with_match(num: u64) -> u64 {
  return match num {
    0 => 1,
    _ => num * factorial_recursive_with_match(num - 1)
  }
}

pub fn factorial_fold(num: u64) -> u64 {
  (1..num+1).fold(1, |acc, n| acc * n)
}