pub fn urutkan_huruf(data: String) {
  let mut i = 0;
  while i < data.len() {
    if let Some(first_char) = data.chars().nth(i) {
      println!("Karakter pertama: {}", first_char);
  } else {
      println!("String kosong");
  }
    println!("{}", i);
    i += 1;
  }
}