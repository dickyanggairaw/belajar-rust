pub fn menghitung_jumlah_kata(kata: String) -> i32 {

  let mut i = 0;

  let mut temp_word = String::from("");
  let mut word_array: Vec<String> = Vec::new();
  while i < kata.len() {
    // let mut first_char_a = ' ';
    if let Some(first_char) = kata.chars().nth(i) {
      println!("{}", first_char);
      if first_char == ' ' {
        word_array.push(temp_word);
        temp_word = String::from("");
      }else {
        temp_word.push(first_char)
      }
    }
    i += 1;
  }
  word_array.push(temp_word);
  println!("Isi vector: {:?}", word_array);
  word_array.len() as i32
}