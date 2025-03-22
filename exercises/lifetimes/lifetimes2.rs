// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        //在 main 函数中，string2 是一个局部变量，其生命周期仅限于内部的块作用域 {}。
        //调用 longest(string1.as_str(), string2.as_str()) 时，longest 函数的返回值的生命周期被绑定到两个输入参数中较短的那个生命周期（即 string2 的生命周期）。
        //当离开内部块作用域后，string2 被销毁，因此 result 引用了一个已经失效的字符串，导致编译错误。
    }
    println!("The longest string is '{}'", result);
}
