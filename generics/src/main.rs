fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let p = Point { x: 5, y: 10 };
    print!("p.x = {}, p.y = {}", p.x, p.y)
}
