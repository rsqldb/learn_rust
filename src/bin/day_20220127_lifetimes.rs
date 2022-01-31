use std::fmt::Display;

//lifetime
fn main() {

    let x = "aaa";
    let y = "bbbc";
    let longest1 = longest1(String::from(x), String::from(y));
    let longest2 = longest(x, y);
    println!("longest1: {}", longest1);
    println!("longest2: {}", longest2);
    let longest3 = longest_with_notification("aaac", "ddddb", "notice");
    println!("longest3: {}", longest3)
}

// compile error:  expected named lifetime parameter
// lifetime parameter is to specify the references lifetime, in this example, the compiler doesn't know
// the parameter lifetime
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

//need to change to
//comments:
//1. lifetime notation won't change the parameter life time
//2. lifetime for single parameter is meaningless, because lifetime notation is aim to solve the lifetime
//differences between multiple parameters.
//'a below is the shorter lifetime of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// single parameter won't have lifetime issue
fn single_pram(x: &str) -> &str {
    x
}

fn longest1(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_notification<'a, T>(x: &'a str, y: &'a str, notice: T) -> &'a str
    where T: Display {
    println!("notify: {}", notice);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 'static is the lifetime of the entire program running
// all &str has the 'static lifetime, "string" is stored in stack

// Any attribute of struct uses reference type must have the lifetime notation
struct ImportantStruct<'a> {
    name: &'a str
}

impl<'a> ImportantStruct<'a> {
    fn print(&self) -> u32 {
        println!("{}", &self.name);
        10
    }
}


#[test]
fn test_largest() {

}