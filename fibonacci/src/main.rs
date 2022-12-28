use std::io;

fn fibonacci(x: u32) -> u32 {
    if x > 1 {
        let y = fibonacci(x-1) + fibonacci(x-2);
        return y;
    } else {
        return x;
    }
}

fn main() {
    let mut input = String::new();
    println!("Input index");
    io::stdin().read_line(&mut input).expect("error");
    let input:u32 = input.trim().parse().unwrap();
    if input <= u32::MAX {
        println!("{}", fibonacci(input));
    } else {
        println!("error");
    }
}
