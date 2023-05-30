fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 函数的返回值如果是一个引用类型，那么它的生命周期只能来源于参数本身或者函数内部的局部变量。
    // 如果是后者，那么就会造成悬垂引用场景
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期总结：标注语法只是告诉编译器引用的关系，它不改变实际的生命周期。
// 并且此处的'a会取2个引用生命周期的最小值，也就是说取2者的交叉区域。
// 生命周期的语法用来将函数的参数和返回值作用于关联到一起，编译器就可以确定操作是否是内存安全的

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 编译器中存在消除规则，只要符合规则，就可以省略生命周期标注