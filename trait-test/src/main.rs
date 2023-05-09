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

// 可以通过函数返回一个类型，该类型实现了某个特征
// 这样可以在函数内部实现特征
// 只能返回一个特征，不能返回多个特征，否则会报错
// 应用场景为：有些类型非常复杂，要自己实现一下很难，用这样的方法可以更简单的返回特征
fn return_summarizable() -> impl Summary {
    Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()}
}

// 一个函数如何返回多个特征类型，就要使用特征对象概念
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());

    
}