use std::any::Any;

fn main() {
    let a = "hhh";
    println!("Hello, world! {} && {}", add(1, 2), a);
    // 测试一下没有使用的变量, 用_解决
    let _b = "";
    // 变量结构
    let (c, d) = (true, false);
    let e = 3;
    let f = 4.5;
    print!("e和f的类型是i32和f64{}和{}", e, f);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回相加值，这里可以省略return
    i + j
}
