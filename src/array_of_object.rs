#[derive()]
pub struct Kendaraan {
  pub merk: String,
  pub cc: u32,
  pub piston: u8
}

pub fn buat_kendaraan(data: Vec<Kendaraan>) {
  for e in data {
      print_item(e);
  }
}

trait Print {
    fn print(&self);
}

impl Print for Kendaraan {
    fn print(&self) {
      println!("punya {} trus cc nya : {} dan punya piston : {}", self.merk, self.cc, self.piston);
    }
}

fn print_item<T: Print> (item: T) {
  item.print();
}