#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 100,
        height: 100
    };
    let rect1 = Rectangle{
        width: 50,
        height: 50
    };
    // 调用关联函数快速创建一个rect
    let rect2 = Rectangle::square(100);
    let result: u32 = rect.area();
    // 查看rect是否可以包含rect1
    let hold_result = rect.can_hold(&rect1);
    println!("rect: {:?}, result: {}", rect, result);
    println!("can_hold {}", hold_result);
    println!("new result {:?}", rect2)
}