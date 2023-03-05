fn greater<T: std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn greater_test() {
    let x = 5;
    let y = 12;

    println!("The greater is {}", greater(x, y));
}

struct Point<T> {
    x: T,
    y: T,
}
//

fn point_test() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x, p.y);
}

fn main() {
    greater_test();
    point_test();
}
