mod urutkan_huruf;
mod palindrome;
mod menghitung_jumlah_kata;
mod menemukan_angka_terbesar;
mod menemukan_character_terbanyak;
mod ubah_angka_ke_binary;
mod hitung_angka_array_of_array;
mod array_of_object;
mod modules;
mod sum_of_digit;
mod square_every_digits;
mod sum_two_smallest_numbers;
mod mumbling;

use array_of_object::{buat_kendaraan, Kendaraan};
use hitung_angka_array_of_array::jumlah;
use menemukan_character_terbanyak::menemukan_character_terbanyak;
use ubah_angka_ke_binary::recursive_modulus_2;
use urutkan_huruf::urutkan_huruf;
use palindrome::palindrome;
use menghitung_jumlah_kata::menghitung_jumlah_kata;
use menemukan_angka_terbesar::menemukan_angka_terbesar;
use sum_of_digit::digital_root;
use square_every_digits::square_digits;
use sum_two_smallest_numbers::sum_two_smallest_numbers;
use mumbling::accum;

fn main() {
    println!("Hello, world!");
    urutkan_huruf(String::from("abcdefg"));

    let first = palindrome(String::from("aqua"));
    println!("first expect is false : {}", first);
    let second = palindrome(String::from("kodok"));
    println!("second expect is true : {}", second);
    let third = palindrome(String::from("rumah"));
    println!("third expect is false : {}", third);
    let fourth = palindrome(String::from("12321"));
    println!("fourth expect is false : {}", fourth);

    let menghitung_jumlah_kata1 = menghitung_jumlah_kata(String::from("Was it a car or a cat I saw?"));
    println!("{}", menghitung_jumlah_kata1);

    let result_angka_terbesar = menemukan_angka_terbesar(vec![1,2,3,4,5,6,7,8]);
    println!("{}",result_angka_terbesar);

    let result_chat_terbanyak = menemukan_character_terbanyak(String::from("aku sayang ibu"));
    println!("{} anka terbanyak", result_chat_terbanyak);

    let result_binary = recursive_modulus_2(4);
    println!("binary dari angka 4 :{}", result_binary);

    let result_jumlah = jumlah(vec![vec![3,4,5,6,8], vec![9,7,3,5]]);
    print!("jumlah : {}", result_jumlah);

    let kendaraan = vec![
      Kendaraan {
        merk: String::from( "Honda Jazz"),
        cc: 1600,
        piston: 4,
      },
      Kendaraan {
        merk: String::from( "Suzuki Swift"),
        cc: 1400,
        piston: 4,
      }
    ];
    buat_kendaraan(kendaraan);

    let result_sum_of_digit = digital_root(16);
    println!("result_sum_of_digit: {}", result_sum_of_digit);
    println!("{}", square_digits(9119));
    println!("{}", sum_two_smallest_numbers(&[19, 5, 42, 2, 77]));
    println!("{}", accum("NyffsGeyylB"));
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn test_palindrom1() {
    let result = palindrome(String::from("Racecar"));
    assert_eq!(result, true)
  }
  #[test]
  fn test_palindrom2() {
    let result = palindrome(String::from("Madam"));
    assert_eq!(result, true)
  }
  #[test]
  fn test_palindrom3() {
    let result = palindrome(String::from("12321"));
    assert_eq!(result, true)
  }
  #[test]
  fn test_palindrom4() {
    let result = palindrome(String::from("Was it a car or a cat I saw?"));
    assert_eq!(result, true)
  }

  #[test]
  fn test_hitung_jumlah_kata1() {
    let result = menghitung_jumlah_kata(String::from("Was it a car or a cat I saw?"));
    assert_eq!(result, 9)
  }

  #[test]
  fn test_hitung_angka_terbesar() {
    let result = menemukan_angka_terbesar(vec![23,50,32,41,51,91]);
    assert_eq!(result, 91)
  }
  #[test]
  fn test_huruf_terbanyak() {
    let result = menemukan_character_terbanyak(String::from("selamat datang dunia"));
    assert_eq!(result, "a")
  }
  #[test]
  fn test_binary(){
    let result = recursive_modulus_2(100);
    assert_eq!(result, "1100100")
  }
  #[test]
  fn test_binary2(){
    let result = recursive_modulus_2(65432);
    assert_eq!(result, "1111111110011000")
  }
  #[test]
  fn test_sum_digit1(){
    let result = digital_root(16);
    assert_eq!(result, 7)
  }
  #[test]
  fn test_sum_digit2(){
    let result = digital_root(942);
    assert_eq!(result, 6)
  }
  #[test]
  fn test_sum_digit3(){
    let result = digital_root(132189);
    assert_eq!(result, 6)
  }
  #[test]
  fn test_sum_digit4(){
    let result = digital_root(493193);
    assert_eq!(result, 2)
  }
  #[test]
  fn test_square_digits() {
    assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
  }
  #[test]
    fn sample_tests() {
        assert_eq!(sum_two_smallest_numbers(&[5, 8, 12, 19, 22]),  13, "Incorrect result for [5, 8, 12, 19, 22]");
        assert_eq!(sum_two_smallest_numbers(&[15, 28, 4, 2, 43]), 6, "Incorrect result for [15, 28, 4, 2, 43]");
        assert_eq!(sum_two_smallest_numbers(&[23, 71, 33, 82, 1]), 24, "Incorrect result for [23, 71, 33, 82, 1]");
        assert_eq!(sum_two_smallest_numbers(&[52, 76, 14, 12, 4]), 16, "Incorrect result for [52, 76, 14, 12, 4]");
        assert_eq!(sum_two_smallest_numbers(&[1, 1, 5, 5]),  2, "Incorrect result for [1, 1, 5, 5]");
    }
    #[test]
    fn basic_tests() {
      assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
      assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
      assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
      assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
      assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
    }
}