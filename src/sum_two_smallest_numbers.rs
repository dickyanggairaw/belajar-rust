pub fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
  let mut numbers_vec: Vec<u32> = numbers.to_vec();
  numbers_vec.sort();
  let mut result: u32 = 0;
  let mut i = 0;
while i < 2 {
  result = result + numbers_vec[i];
    i = i + 1;
}
  result
}