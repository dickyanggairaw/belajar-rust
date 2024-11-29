pub fn accum(s:&str)->String {
  let mut result = String::from("");
  let mut i = 0;
  while i < s.len() {
    let temp = i + 1;
    let mut j = 0;
      while j < temp {
          if let Some(first_char) = s.chars().nth(i) {
            if j == 0 {
              result.push_str(&format!("{}", first_char.to_uppercase()));
            }else {
              result.push_str(&format!("{}", first_char.to_ascii_lowercase()));
            }
          }
          j = j+1;
      }
    if i != s.len() - 1 {
      result.push('-');
    }
    i = i + 1;
  }
  result
}