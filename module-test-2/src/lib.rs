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

mod front_of_house;

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

use std::collections::{HashMap,BTreeMap,HashSet};
