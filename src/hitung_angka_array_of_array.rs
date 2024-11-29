pub fn jumlah(data: Vec<Vec<u32>>) -> u32 {
  let mut result = 0;

  for numbers in data {
    for number in numbers {
      result += number;
    }
  }

  result
}