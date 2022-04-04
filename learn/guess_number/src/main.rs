use rand::Rng;
use std::io;

fn main() {
    println!("Start the Game!");

    let target_number: u32 = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your numbrer!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue,
        };

        println!("you guess {}", guess_num);
        if target_number == guess_num {
            println!("Got it!");
            break;
        } else if target_number < guess_num {
            println!("too big!");
        } else {
            println!("too small!");
        }
    }
}
