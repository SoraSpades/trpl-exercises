use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        print!("Enter guess: ");
        io::stdout().flush().expect("Error Flushing stdout");
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Bigger"),
            Ordering::Greater => println!("Smaller"),
            Ordering::Equal => { 
                println!("Win");
                break;
            }
        }
    }
}
