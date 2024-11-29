fn remove_non_alphabetic(input: &str) -> String {
  input
      .chars()
      .filter(|c| c.is_alphanumeric())
      .collect()
}

pub fn palindrome(char: String) -> bool {
  let char_clone = remove_non_alphabetic(&char).to_lowercase();
  let mut result: bool = false;
  let mut palindrome_char = String::from("");

  //code here

  let len = char_clone.len();
  let mut i = len as i32;

  while i >= 0 {
    let usize_value: usize = i as usize;
    if let Some(first_char) = char_clone.chars().nth(usize_value) {
        palindrome_char.push(first_char);
    } else {
    }
    i -= 1;
  }
  if palindrome_char == char_clone {
    result = true;
  }
  result
}