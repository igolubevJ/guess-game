use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число равно {}", secret_number);

    println!("Пожадуйста, введите свою догадку.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Не получилось прочитать строку");

    let guess: u32 = guess.trim().parse()
        .expect("Пожалуйста, набирите число!");

    println!("Вы загадали: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Слишком малое число!"),
        Ordering::Greater => println!("Слишком большое число!"),
        Ordering::Equal => println!("Вы выйграли!"),
    }
}
