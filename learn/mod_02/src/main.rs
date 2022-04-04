// 注意 mod 是模型，use 是 crate
// 导入模块
mod mod1;
mod mod2;

fn main() {
    println!("{}", mod1::MESSAGE);
    mod2::hello::say_hello();
}
