use rand::Rng;
use std::cmp::Ordering;
use std::io;

const LOWEST_NUMBER: u32 = 1;
const HIGHEST_NUMBER: u32 = 100;

fn main() {
  println!("Guess the number! Between {} & {} included", LOWEST_NUMBER, HIGHEST_NUMBER);

  let secret_number = rand::thread_rng().gen_range(LOWEST_NUMBER, HIGHEST_NUMBER + 1);  // First included, second not

  // println!("The secret number is: {}", secret_number);

  let mut left_attempts = 7;

  while left_attempts > 0 {
    println!("[{} attempts left] Please input your guess: ", left_attempts);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("You did not input a number, please input again");
        continue;
      },  // _ Catches all errors, no matter what information they have inside
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
      Ordering::Greater => println!("Too big!")
    }

    left_attempts -= 1;
  }

  if left_attempts <= 0 {
    println!("You ran out of attempts! The number was: {}", secret_number);
  }
}
