use std::thread;

fn main() {
    test_if_else();
    test_loop();

    test_match();

    // function
    test_fabonacci();
    // method
    test_method();

    test_closure();

    test_thread();

    test_func_input_output();
}

fn test_if_else() {
    let n = 1;
    if n < 0 {
        println!(" n is < 0");
    } else {
        println!(" n is > 0");
    }

    let a = if n <= 1 { 10 } else { 100 }; // 这个分号是 a = 表达式的结束标识
    println!("a = {} ", a);
}

fn test_loop() {
    let mut sum = 0;
    let mut a = 0;
    loop {
        if a > 100 {
            break;
        }
        sum += a;
        a += 1;
    }
    println!("sum = {}", sum);
}

fn test_match() {
    let a = 101;
    match a {
        100 => {
            println!("a is 100 ");
        }

        _ => {
            println!("other number!")
        }
    }
}

// if 和 else 由于分号的是表达式，才可以作为返回值
fn fabonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fabonacci(n - 1) + fabonacci(n - 2)
    }
}

fn test_fabonacci() {
    let f10 = fabonacci(10);
    println!("fabanacci 10 = {} ", f10);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // 构造方法
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}

fn test_method() {
    let p = Point::new(10, 100);
    println!("p : {:?}, p.x = {}, p.y={}", p, p.get_x(), p.get_y());
}

// 闭包
fn test_closure() {
    let mul3 = |n: i32| -> i32 { n * 3 };
    println!("mul3(3) = {} ", mul3(3));
}

// 多线程, 闭包
fn test_thread() {
    let hello = "hello world";

    thread::spawn(move || {
        println!("{} ", hello);
    })
    .join();
}

// 函数作为输入参数
// 作为返回值
type Func = fn(i32, i32) -> i32;
fn getFunc(funcStr: &str) -> Func {
    match funcStr {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}

fn cal(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn test_func_input_output() {
    let a1 = cal(add, 100, 10);
    let a2 = cal(sub, 100, 10);
    println!("100, 10, add={}, sub={}", a1, a2);
    println!("{}", getFunc("add")(1, 1));
}
