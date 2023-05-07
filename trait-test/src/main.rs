pub trait Summary {
    fn summarize(&self) -> String;   
}

pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

use std::fmt::Display;

pub struct Weibo {
    pub username: String,
    pub content: String
}


// 实现特征
impl Summary for Post {
    fn summarize(&self) -> String {
       format!("文章{}, 作者是{}", self.title, self.author)
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("通知：{}", item.summarize());
}

// 这样只是一个语法糖，实际上是下面的写法
// 对于特征约束，下面的写法更加灵活，语法糖写法针对于不同的特征很好用
pub fn notify2<T: Summary>(item: T) {
    println!("通知：{}", item.summarize());
}

// 多重约束的2种写法
pub fn notify3(item: impl Summary + Display) {
    println!("通知：{}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: T) {
    println!("通知：{}", item.summarize());
}

// 函数签名如果特征约束很复杂，可以通过where
pub fn notify5<T, U>(item1: T, item2: U) -> i32
    where T: Summary + Display,
          U: Summary + Display
{
    println!("通知：{}", item1.summarize());
    println!("通知：{}", item2.summarize());
    1
}

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}