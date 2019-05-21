// （输入/输出）库引入当前作用域
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    // 创建一个储存用户输入的地方
    // 这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上
    let mut guess = String::new();

    // .read_line(&mut guess)，调用 read_line 方法从标准输入句柄获取用户输入

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
