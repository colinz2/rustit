// 生命周期的注解
//  fn str_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//|              ++++      ++           ++          ++
fn str_bigger<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

fn str_bigger_test() {
    let s1 = "Hello";
    let s2 = "World";

    println!("The bigger string is {}", str_bigger(s1, s2));
}

// 统一生命周期
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    str_bigger_test();    
}
