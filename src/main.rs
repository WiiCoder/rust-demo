use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    test();
}

fn test() {
    // 通常情况下 {} 会被任意变量替代，31默认为 i32类型，31i64 则为 i64 类型
    println!("{} days", 31);

    // 可以使用位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以命名参数
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");
    
    // :后指定特殊格式，b为二进制
    println!("{} of {:b} people know binary, the other half don't",1,2);

    // 填充
    println!("1{number:>width$}", number=1,width=5);
    println!("{number:>0width$}", number=1,width=6);

    /*
    如下代码会出现的错误，^ 标识错误位置
    println!("My name is {0}, {1} {0}", "Bond");
    error: invalid reference to positional argument 1 (there is 1 argument)
    --> src/main.rs:37:32
       |
    37 |     println!("My name is {0}, {1} {0}", "Bond");
       |                                ^
       |
       = note: positional arguments are zero-based
    */
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // 创建一个包含单个 `i32` 的结构体。
    #[allow(dead_code)]
    struct Structure(i32);
    // 自定义类型的结构体无法使用如下语句执行
    // println!("This struct `{}` won't print...", Structure(3));
    // 可使用如下语句
    println!("This struct `{:?}` won't print...", Structure(3).0);


    // 再用一个 println! 宏，通过控制显示的小数位数来打印：Pi is roughly 3.142（Pi 约等于 3.142）。
    // 为了达到练习目的，使用 let pi = 3.141592 作为 Pi 的近似值
    let pi = 3.141592;
    println!("Pi is roughly {:.3}",pi)
}