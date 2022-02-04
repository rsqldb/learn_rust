fn main() {
    match_arms();
    if_let_use();
    while_let_use();
    for_loop_use();
    fn_parameter_use(&(10, 20));
    match_with_guards();
}

fn match_arms() {
    let option: Option<&str> = Some("aaa");

    match option {
        Some(str) => println!("{}", &str),
        _ => println!("else")
    };
}

fn if_let_use() {
    let option = Some("aaa");
    let age = Some(10);
    if let Some(str) = option {
        println!("{}", &str);
    } else if let Some(val) = age {
        println!("{}", val);
    } else {
        println!("else")
    }
}

fn while_let_use() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(val) = stack.pop() {
        println!("val: {}", val);
    }
}

fn for_loop_use() {
    let vec = vec!["a", "bb", "cc", "ddd"];

    for (index, val) in vec.iter().enumerate() {
        println!("val in index: {}: {}", index, val);
    }
}

fn let_use() {
    let x = 10;
    let (x, y) = (10, 22);
}

fn fn_parameter_use(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

///Refutability: Whether a Pattern Might Fail to Match
///patterns after let is irrefutable
///
/// Extra Conditionals with Match Guards
fn match_with_guards() {
    let option = Some(6);
    match option {
        Some(x) if x < 5 => println!("{}", x),
        Some(1..=6) => println!("x is in 1..6"),
        Some(x) => println!("x: {}", x),
        _ => println!("else"),
    };
}