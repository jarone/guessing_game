use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字游戏欢迎您!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_times = 0;

    loop {
        println!("请输入您猜的数字（1到100之间，包含1和100）");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_times = guess_times + 1;
        println!("您猜的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("您猜中了! 共猜了 {} 次", guess_times);
                break;
            }
        }
    }
}
