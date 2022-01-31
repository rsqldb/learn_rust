use std::fs::File;
fn main() {

    let f = 0;
    //From the below example we can see that the condition
    let f = if f == 0 { 10 } else { 5 };
    // let f = if let f0 = f  { 20 };
    let f = match f {
        10 => 20,
        _ => 0
    };

    let a = Foo::Bar;

    if let Foo::Bar = a {
        println!("match")
    }

    let file = File::create("hello.txt");
}


enum Foo {
    Bar
}