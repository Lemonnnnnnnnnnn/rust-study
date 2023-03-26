use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut guess = String::new();

    io::stdin()
    // read_line: 无论用户在标准输入中键入什么内容，都将其追加（不会覆盖其原有内容）到一个字符串中，
    // 因此它需要字符串作为参数
        .read_line(&mut guess) 
        .expect("Failed to read line");

    println!("You guessed : {guess}");
    
}
