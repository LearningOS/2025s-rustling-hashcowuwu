// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
        };
        //在 macro_rules! 宏定义中，每个分支（即每条规则）之间需要用分号 (;) 分隔。这是因为 Rust 的宏解析器需要明确区分不同的分支规则。如果没有分号，编译器会报错，因为它无法正确解析宏定义。
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
