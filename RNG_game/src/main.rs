use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num = rand::rng().random_range(1..=100);
    println!("generated num {rand_num}");
    loop {
        println!("Choose a random number");
        let mut ip = String::new();
        io::stdin().read_line(&mut ip).expect("unable to read line");

        let ip: i32 = match ip.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        println!("your choice: {ip}");
        match ip.cmp(&rand_num) {
            Ordering::Less => println!("Your number is less than the generated number."),
            Ordering::Greater => println!("Your number is greater than the generated number."),
            Ordering::Equal => {
                println!("Your number is equal to the generated number.");
                println!("Closing...");
                break;
            }
        }
    }
}
