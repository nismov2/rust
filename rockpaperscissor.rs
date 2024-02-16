use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Let's play Rock, Paper, Scissors!");
    loop {
        println!("Please enter your choice (rock, paper, or scissors):");
        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice).expect("Failed to read line");
        let user_choice = user_choice.trim().to_lowercase();

        if user_choice != "rock" && user_choice != "paper" && user_choice != "scissors" {
            println!("Invalid choice. Please enter rock, paper, or scissors.");
            continue;
        }

        let computer_choice = match rng.gen_range(0..3) {
            0 => "rock",
            1 => "paper",
            _ => "scissors",
        };

        println!("Computer chose {}", computer_choice);

        match (user_choice.as_str(), computer_choice) {
            ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => {
                println!("You win!");
            }
            ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => {
                println!("Computer wins!");
            }
            _ => {
                println!("It's a tie!");
            }
        }

        println!("Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        if play_again.trim().to_lowercase() != "yes" {
            break;
        }
    }
}
