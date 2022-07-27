use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("游戏结束🎮");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    // 生成一个1-100随机值
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("秘密数字已生成, 请在1-100中猜一个数字，输入非法字符会自动结束");
    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数字是: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("你猜对了");
                break;
            },
            Ordering::Greater => println!("你猜的有点大oh～，再试试吧"),
            Ordering::Less => println!("你猜的有点小oh～，再试试吧"),
        }
    }
}
