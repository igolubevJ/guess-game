use std::io;

fn main() {
    println!("Угадай число!");

    println!("Пожадуйста, введите свою догадку.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Не получилось прочитать строку");

    println!("Вы загадали: {}", guess);
}
