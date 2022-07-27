fn main() {
    // 测试一下let和const区别
    let mut a = "1";
    println!("{}", a);
    // 更改一下a
    a = "2";
    println!("{}", a);
    // 尝试一下const，必须大写
    const B: u32 = 60 * 60;
    println!("{}", B);
    // 通过函数赋值给const变量
    // 看看表达式概念，用{}创建一个表达式
    let y = {
        let x = 1;
        x + 1
    };
    println!("{}", y);
    // 可以定义一个函数，函数通过表达式直接返回x+1，并且赋值给ab
    let ab = t(4);
    println!("{}", ab);
    // if表达式
    if true {
        println!("if表达式为真");
    }else{
        println!("if表达式为假");
    }
    // 循环，无限循环
    // loop {
    //     println!("again!");
    // }
    // 循环标签
    'counter: loop{
        break 'counter;
    }
    // 从循环返回值
    let loop_result = loop {
        break 'n';
    };
    println!("{}", loop_result);
    // 也可以使用while循环，省去了很多if else结构代码
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
    // 如果用while对数组进行循环，也不是不可以，可以通过index来访问数组，这会更慢，编译器要加运行时代码每次判断索引是否在边界之内
    // 我们可以使用for循环，来对数组进行迭代，这会更清晰
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn t(x: i32) -> i32 {
    x + 1
}