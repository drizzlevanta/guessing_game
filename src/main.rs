use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut is_game_over = false;
    let rn = generate_rn();

    let guess = 374;

    while !is_game_over {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("guess is {}", &guess);

        match guess.cmp(&rn) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                is_game_over = true
            }
        }
    }

    println!("Game over! Guess is {}", &guess);
}

fn generate_rn() -> u32 {
    let mut rng = rand::thread_rng();

    let rn = rng.gen_range(1..=100);
    println!("generated random number is {}", rn);
    rn
}

// fn accept_input() -> u32 {
//     println!("Enter your guess:");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     let guess: u32 = guess.trim().parse().expect("Input not an integer");
//     println!("guess is {}", &guess);
//     guess
// }

// fn check_input(num: &u32, guess: &u32) {
//     if num == guess {
//         println!("Congrats! Correct guess!");
//         return;
//     }

//     if num > guess {
//         println!("Guess is too low!");
//         return;
//     }

//     if num < guess {
//         println!("Guess is too high!");
//     }
// }
