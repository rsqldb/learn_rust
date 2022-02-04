// Rust smart pointer
///
/// Difference between reference and smart pointer: reference just borrows the object,
/// but smart pointer owns data most of the time
/// String and Vec<T> are smart pointers actually
/// Smart pointers are struct in rust and implemented Deref trait and Drop trait
/// Deref trait allows smart pointer being used as references
/// Drop trait allows smart pointer defining destructure code

/// Box<T> the most simple smart pointer, it allocate data on heap
/// Box can be used to solve recursion problems
///
fn main() {
    create_in_box();
    deref_case();
    deref_coercion();
    rec_use();
    ref_cell_use();
    rc_ref_cell_use();
}

use crate::List::{Nil, Cons};

fn create_in_box() {
    let b = Box::new(5);
    println!("box value: {}", b);
    // let list = List::Cons(1,
    //                       List::Cons(2,
    //                                     List::Cons(3,
    //                                                List::Cons(4,
    //                                                           List::Nil))));

    let _list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}

///Can't define recursive like this, compile error: recursive type has infinite size
// enum List {
//     Cons(i32, List),
//     Nil
// }

enum List {
    Cons(i32, Box<List>),
    Nil
}
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil
}

use std::ops::Deref;
//tuple struct
struct MyBox<T>(T);

//Customized Smart Pointer, which implemented Deref trait
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    ///destruct method
    ///can't be called explicitly
    /// If want to drop memory first, can use std::mem::drop
    fn drop(&mut self) {
        println!("Drop myBox!!")
    }
}

fn deref_case() {
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/// 隐式解引用转换
/// No run cost, because it's done in compile stage
///
/// If implementing DerefMut trait, Rust would do deref coercion for the below cases:
/// 1. if T:Deref<Target=U>, allow &T to &U
/// 2. if T:DerefMut<Target=U>, allow &mut T to &mut U
/// 3. if T:Deref<Target=U>, allow &mut T to &U
fn deref_coercion() {
    hello("Rust");
    let m = MyBox::new(String::from("drew"));

    //compiler call deref to convert to coordinate types
    //&m is &MyBox<String>
    //call deref to &String
    //call deref to &str
    hello(&m);
    //if no coercion, it should be
    hello(&(*m)[..])
}

fn hello(name: &str) {
    println!("hello, {}", name);
}

use std::rc::Rc;
/// Rc<T> - Reference counting, rust maintain a counter for all references, when references become 0, then system clean it.
/// Scenarios of using Rc<T>:
/// 1. Need to use data in heap, and it needs to be read in multiple place and it can't be decided when
/// the last reading happens.
/// 2. Rc<T> can only be used in single thread
/// Rc<T> is immutable reference way to share data between different places
///
/// sample: create two list, both use another list
fn rec_use() {

    //Not working, because a is moved
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(RcList::Cons(5,
                         Rc::new(RcList::Cons(10,
                                         Rc::new(RcList::Nil)))));
/// differences between a.clone and Rc::clone(&a):
/// Rc::clone just increase the counter, don't do deep copy while a.clone might do

    println!("counter after a = {}", Rc::strong_count(&a));
    //Rc::clone increase the reference counter from 1 to 2
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("counter after a = {}", Rc::strong_count(&a));
    //Rc::clone increase the reference counter from 2 to 3
    {
        let c = RcList::Cons(5, Rc::clone(&a));
        println!("counter after a = {}", Rc::strong_count(&a));
    }
    println!("counter after a = {}", Rc::strong_count(&a));

}

use std::cell::RefCell;
/// Box<T>: enforce borrow rule in compile stage, can have one owner only, can be mutable and immutable
/// RefCell<T>: check borrow rule in run stage, can have one owner only, can be mutable and immutable
/// RefCell<T> can be used in single thread only, same as Rc<T>
/// Rc<T>: can have multiple owners, can be immutable only
///
/// For RefCell<T>
/// Has two methods:
/// borrow returns Ref<T>
/// borrow_mut returns RefMut<T>
///
/// At anytime, rust only allow multiple immutable borrows or one mutable borrow.
///
fn ref_cell_use() {
    let a = vec![1, 2, 3, 4];
    let b = RefCell::new(a);
    b.borrow_mut().push(10);
    println!("vec {:?}", b.borrow());
}

///
/// Rc<RefCell<T>> to implement multiple owners mutable data.
/// Rc<T> allows multiple references with reference counter, but T can be only immutable, then
/// if wrapper RefCell in Rc, which means it T can be mutable
///
///
/// Other mutable types
/// Cell<T>: access data by copying
/// Mutex<T>:
///
fn rc_ref_cell_use() {
    let a = vec![1, 2, 3, 4, 5, 6];
    let b = Rc::new(RefCell::new(a));
    let d = Rc::clone(&b);
    let e = Rc::clone(&b);
    b.borrow_mut().push(0);
    println!("vec b: {:?}", b.borrow());
    d.borrow_mut().push(10);
    println!("vec d: {:?}", d.borrow());
    e.borrow_mut().push(11);
    println!("vec d: {:?}", d.borrow());
    println!("vec b: {:?}", b.borrow());
}

/// Memory leak
/// Rc<T> and RefCell<T> circle reference could cause memory leak
