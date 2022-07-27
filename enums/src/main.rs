#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 定义方法
impl IpAddr {
    fn call() {
        // 在这里定义方法体
        println!("hello")
    }
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 使用match，rust中的匹配是穷举，必须要添加不必要的样板代码，使编译不报错，我们可以使用其他的语法来解决这个问题
// if let Some(3) = some_u8_value {
//     println!("three");
// }

// 要么语法简洁，要么失去穷尽特质，2者的使用还要看具体的环境做出取舍

fn main() {
    let a = IpAddr::V4(127, 0, 0, 1);
    let b = IpAddr::V6(String::from("::1"));
    let some_number = Some(5);
    println!("{:?}", some_number);
    IpAddr::call();
    println!("{:?}", a);
}
