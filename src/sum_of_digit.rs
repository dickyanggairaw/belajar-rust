pub fn digital_root(n: i64) -> i64 {
  recursive(n)
}

fn recursive(n: i64) -> i64 {
  if n < 10 {
    return n;
  } else {
    let string = n.to_string();
    let mut i = 0;
    let mut temp: i64 = 0;
    while i < string.len() {
        if let Some(first_char) = string.chars().nth(i) {
          let num = String::from(first_char).parse::<i64>();
          temp = temp + num.unwrap();
        }else {
          temp = temp + 0;
        }
      i = i+1;
    }
    return recursive(temp);
  }
}