fn main() {
    println!("------");
    func_test();

    println!("------");
    array_test();

    println!("------");
    struct_test();

    println!("------");
    enum_test();
}

fn arvg(a: u32, b: u32) -> u32 {
    let arvg = (a + b) / 2;
    return arvg;
}

fn func_test() {
    println!("{}", arvg(100, 100));
}

fn array_test() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[1] = {:?}", a[1]);
}

fn struct_test() {
    // 元组结构
    struct Pair(i32, f32);
    let pair = Pair(10, 32.11);
    println!("pair.0 = {}, pair.1 = {}", pair.0, pair.1);

    // 标准 C 结构
    // how to print a whole strcut, 打印的时候加上 :? 
    // derive 派生属性
    #[derive(Debug)]
    struct Persion {
        name: String,
        age:  u32,
    }

    let jack = Persion{
        name : String::from("jack ma"),
        age  : 6,
    };
    println!("jack name: {}, jack age: {} ", jack.name, jack.age);
    println!("jack is {:?} ", jack);

    // 单元结构
    // 泛型中使用
    // struct Unit;
    // let unit = Unit;
    
}

// 枚举类型 与 match 语句匹配
fn enum_test() {
    // 带枚举
    // enum Color {
    //     Red = 11,
    //     Green = 12,
    //     Blue = 13,
    // }
    
    // 带参数
    enum IpAddr {
        IpV4(u8, u8, u8, u8),
        IpV6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)
    }

    let localhost: IpAddr = IpAddr::IpV4(127, 0, 0, 1);
    match localhost {
        IpAddr::IpV4(a, b, c, d) => {
            println!("ipv4 {}.{}.{}.{}", a, b, c, d);
        }   

        IpAddr::IpV6(..) => {
            println!("IpV6");
        }

        _ => {
            println!("do not know!~");
        }
    }
        
}
