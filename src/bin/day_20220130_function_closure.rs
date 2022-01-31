use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    //closure parameter type and return type is not required, but it can be set as well
    let expensive_closure = |num| {
        println!("calculating slowly..., input num is {}", num);
        thread::sleep(Duration::from_millis(20));
        num
    };

    println!("{}", expensive_closure(10));
    // println!("{}", expensive_closure("aaa"));
    let mut int_cache = Cache::new(|arg|{
        println!("calculate for key: {}", arg);
       format!("key is {}", arg)
    });
    println!("result: {}", int_cache.value(10));
    println!("result: {}", int_cache.value(10));
    println!("result: {}", int_cache.value(101));

    //Closure can use local variable
    let p = vec!["ex"];
    let function = |x| format!("{}-{:?}", x, p);
    println!("p: {:?}", p);
    println!("function value: {}", function("txt content"));

    //Closure can use move to get the ownership
    let p1 = vec!["ex"];
    let function = move |x| format!("{}-{:?}", x, p1);
    // println!("p: {:?}", p1); // can't borrow after move
    println!("function value: {}", function("txt content"));


    //Iterator don't execute anything if it's not used
    let vector = vec!["a", "b", "c"];
    //Must be mut if want to iterate next
    let mut vec_iter = vector.iter();
    assert_eq!(vec_iter.next(), Some(&"a"));

    //don't need mut, because for moved the ownership of vec_iter
    for entry in vec_iter {
        println!("item: {}", entry)
    }

    //get error info: value borrowed here after move!!
    //println!("any result: {:?}", vec_iter.next())

    //Different iterator method
    //iter: create iterator on immutable value reference
    //into_iter: create iterator own value
    //iter_mut: crete iterator on mutable value reference


    let vector = vec![1, 2, 3];
    let iter: i32 = vector.iter().sum();
    println!("sum: {:?}", iter);

    let mapped_vec:Vec<_> = vector.iter().map(|x| x + 1).collect();
    mapped_vec.iter().for_each(|x| println!("item: {}", x));
    let v = 2;
    let filtered_vec: Vec<_> = mapped_vec.into_iter().filter(|x| *x > v).collect();
    filtered_vec.iter().for_each(|x| println!("filtered item: {}", x));

    zip_iterate();
}



struct Cache<T>
    where T: Fn(u32) -> String {
    closure: T,
    value_map: HashMap<u32, String>,
}

impl<T> Cache<T> where T: Fn(u32) -> String {
    fn new(closure: T) -> Cache<T> {
        Cache {
            closure,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, key: u32) -> &str {
        match self.value_map.get(&key) {
            None => {
                self.value_map.insert(key, (self.closure)(key));
            },
            Some(_v) => (),
        }
        self.value_map.get(&key).unwrap()
    }

}

struct Counter {
    counter: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { counter: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < 5 {
            self.counter += 1;
            Some(self.counter)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn zip_iterate() {
    //Rust rust zero cost abstractions, abstraction code won't affect performance, but having better
    // performance sometimes
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();


    println!("calculated result: {}", sum)
}