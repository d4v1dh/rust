use std::io;

fn get_input() -> (f64, u8) {
   println!("1: C to F");
   println!("2: F to C");
   let mut temp = String::new();
   let mut input = String::new();
   println!("Select option:");
   loop {
      //same as let stdin = io::stdin(); stdin.read_line()
      io::stdin().read_line(&mut input).expect("cannot read value");
      //convert string to u32
      let input = input.trim();
      if input == "1" || input == "2" {
         println!("Enter temp to convert");
         io::stdin().read_line(&mut temp).expect("cannot read value");
         return (temp.trim().parse().unwrap(), input.parse().unwrap());
      }
      println!("Select option:");
   }
}

fn main() {
   let (temp, opt) = get_input();
   match opt {
      1 => println!("{} F", (temp * 1.8 + 32.0).floor()),
      2 => println!("{} C", ((temp - 32.0) * 5.0 / 9.0).floor()),
      _ => println!("error")
   }
}
