use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let res = fib(5).to_string();
    println!("fib5 = {res}");
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input number");
                continue;
            }
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn fib(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    let mut c = 0;
    (0..n).for_each(|_| {
        c = a + b;
        a = b;
        b = c;
    });
    c
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn test(){
    let s = "Hello, world!";
    let s = String::from("Hello, world!");
    let s1 = &s[..];
}