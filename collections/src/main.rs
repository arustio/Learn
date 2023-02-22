use std::collections::HashMap;

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
    // string字符串其实就是字节的集合
    let mut s = String::new();
    let data = "initial contents";
    let mut s = data.to_string();
    // 拼接字符串
    s.push_str("foo");
    s.push('l');
    // 也可以使用 + 来拼接字符串
    let mut s = s + "bar";
    println!("{}", s);
    // + 操作其实是一个add函数
    // fn add(self, s: &str) -> String {
    // 第一个参数其实是借用，所以s之后就会被释放，第二个参数是一个引用，所以不会被释放
    // 如果我要拼接多个字符串，可以优雅的使用format宏
    let  s = format!("{s}-{data}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
