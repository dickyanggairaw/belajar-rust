pub fn square_digits(num: u64) -> u64 {
  let str = num.to_string();
  let mut i = 0;
  let mut temp_result: String = String::from("");
  while i < str.len() {
      if let Some(first_char) = str.chars().nth(i) {
        let num = String::from(first_char).parse::<u32>().unwrap();
        let temp = &num * num;
        let string_num = temp.to_string();
        temp_result.push_str(&format!("{}", string_num));
        println!("temp result: {}", temp_result)
      }else {
        println!("error");
      }
      i = i+1
  }
  let result = temp_result.parse::<u64>().unwrap();
  result
}