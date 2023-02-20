pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod front_of_house {
    pub mod hosting {

        // 给枚举设置pub时，其成员会自动设置为pub
        pub enum Appetizer {
            Soup,
            Salad,
        }

        // 给结构体设置pub时，其字段不会自动设置为pub
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        pub fn add_to_waitlist() {
            print!("add_to_waitlist");
            super::hosting::seat_at_table();
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 使用一个别名来引用路径, 也可以加入pub关键字，让其他人使用包时，可以直接使用hosting作用域
use crate::front_of_house::hosting as HaoHosting;
// 也可以使用嵌套路径
use std::io::{self, Write};
// 以及global
use std::collections::*;

pub fn eat_at_restaurant() {
    // 绝对路径
    HaoHosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}