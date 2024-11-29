use std::collections::HashMap;

pub fn menemukan_character_terbanyak(kata: String) -> String {
  let mut char_count = HashMap::new();

    // Hitung frekuensi kemunculan setiap karakter
    for c in kata.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // Temukan karakter dengan frekuensi tertinggi
    let mut result = ' ';
    let mut angka_terbanyak = 0;

    for (c, count) in char_count {
        if count > angka_terbanyak {
            angka_terbanyak = count;
            result = c;
        }
    }

  result.to_string()
}