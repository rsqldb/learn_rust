
/// in rust, struct and enum should not be called object, as their impl are separated
/// trait object is similar to oop object, but can't define data itself
/// trait object uses dynamic dispatch while generic uses static dispatch, which is replaced while compiling
///
///
/// Constrains in rust to make sure trait object safety:
/// 1. trait methods return type can't be self
/// 2. trait methods can't have generic parameters
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox{
                height: 54,
                width: 34,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                ]
            }),
            Box::new(Button{
                height: 30,
                width: 23,
                label: String::from("Submit"),
            }),
        ]
    };
    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

/// Differences between T program, for T, it can be one specific Type, as it's static
/// pub struct Screen<T: Draw> {
///   pub components: Vec<T>

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button!");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select box!");
    }
}


