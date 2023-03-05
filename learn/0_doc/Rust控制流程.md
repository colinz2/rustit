
#### 函数与闭包
```rust
use std::thread;

fn main() {
    let msg: &str = "hell world";
    // move 将变量转移到闭包函数
    thread::spawn(move || {
        println!("{}", msg);
    })
    .join()
    .unwrap();
}
```

#### 高阶函数
```rust
type Method = fn(u32,u32)->u32;

fn add(a:u32, b:u32) ->u32 {
    a + b
}

fn cal(method:Method, a:u32, b:u32) -> u32 {
     method(a, b)
}

fn main() {
    let x = cal(add, 1, 2);
    print!("{}", x);
}
```

#### 发散函数
发散函数是指永远也不会返回的函数， 他们的返回值被标志为 ！， 这是个空类型。
实际中很少用到。
```rust
fn foo() -> ! {
    panic!("this call never returns!")
}

fn main() {
    let a = if true { 10 } else { foo() };
    print!("{}", a);
}
```

