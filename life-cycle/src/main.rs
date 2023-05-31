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
// 1. 每一个引用参数都会获得独自的生命周期
// 2. 如果只有一个输入生命周期，那么输出生命周期就是这个输入生命周期
// 3. 如果存在多个输入生命周期，其中一个是&self或者&mut self，那么&self的生命周期就是所有的输出生命周期
// 编译器就想管道一样，从第一个规则到第三个规则为代码添加输入生命周期和输出生命周期，如果因为遇到一些错误没办法自动添加，就需要我们手动添加生命周期了

struct ImportantExcerpt<'a> {
    part: &'a str,
}


// 'a: 'b 是生命周期约束语法，说明'a必须比'b活得久
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 在生命周期中存在一个特殊的生命周期：'static，它代表整个程序的生命周期，所有的字符串字面值都拥有'static生命周期
// 在之前学习的字符串是被硬编码到二进制文件中，那么这一类值的生命周期都是'static
// 理论上我们可以给很多生命周期编译不通过的代码都加上'static，只要你保证这不是一个悬垂引用或者引入了潜在的bug
// 存在即合理，希望我们能够合理运用'static生命周期