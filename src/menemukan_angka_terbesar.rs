pub fn menemukan_angka_terbesar(data: Vec<i32>) -> i32{
  let mut result = 0;

  for row in data {
    if result < row {
      result = row;
    }
  }

  result
}