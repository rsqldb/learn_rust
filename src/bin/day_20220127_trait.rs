fn main() {
    println!("largest value: {}", largest(&[10,2]).unwrap());
    let p = Point {x: 5, y: 10};
    println!("{}", p.x1());
    let article = Article { title: String::from("title"), content: String::from("content") };
    println!("article summary: {}", article.summarize());
    let result = notify4(article);

}

fn largest(list: &[i32]) -> Result<i32, std::io::Error> {
    if list.len() == 0 {
        panic!("list is empty!")
    }
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    Ok(largest)
}

trait Summary {
    fn summarize(&self) -> String;
}

trait Display {

}

struct Article {
    title: String,
    content: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} {}", &self.title, &self.content)
    }
}

impl Display for Article {

}

fn notify1(item: impl Summary + Display) {
    println!("notify {}", item.summarize())
}

//the same as above, but looks more concise, - trait bound
fn notify2<T: Summary + Display>(item: T) {
    println!("notify2 {}", item.summarize())
}

//Use where to constraint trait type
fn notify3<T>(item: T) -> String
    where T:Summary + Display,
{
    println!("notify2 {}", item.summarize());
    "".to_string()
}

//Trait as return type
fn notify4<T>(item: T) -> impl Summary
    where T:Summary + Display,
{
    println!("notify2 {}", item.summarize());
    item
}

fn get_largest<T: Copy + PartialOrd + std::fmt::Display>(list: &[T]) -> T {
    let mut largest = list[0];
    println!("largest {}", largest);
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x1(&self) -> &T {
        &self.x
    }
}

enum Type<T, U> {
    Boy(T),
    Girl(U),
}

enum Option<T> {
    Some(T),
    None,
}

enum Error<T, E> {
    Ok(T),
    Err(E),
}

#[test]
fn test_largest() {
    assert_eq!(largest(&[10,2]).unwrap(), 10);
    let number_list = vec![1,23,42,34,32];
    assert_eq!(largest(&number_list).unwrap(), 42);
    assert_eq!(get_largest(&["bca", "bcd"]), "bcd")
}