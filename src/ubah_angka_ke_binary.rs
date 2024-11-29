pub fn recursive_modulus_2(data: u32) -> String {
  if data == 0 {
      return String::new();
  }
  let mut result = recursive_modulus_2(data / 2);
  result.push_str(&(data % 2).to_string());
  result
}