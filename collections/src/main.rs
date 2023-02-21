fn main() {
    println!("Hello, world!");
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // 读取vec的值
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // get方法读取vec的值
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    for i in &v {
        println!("{i}");
    }
}
