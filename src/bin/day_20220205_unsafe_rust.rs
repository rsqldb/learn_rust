/// What unsafe can do?
/// Dereference a raw pointer
/// Call an unsafe function or method
/// Access or modify a mutable static variable
/// Implement an unsafe trait
/// Access fields of unions
///
/// It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked.
fn main() {
    dereference_raw_pointer();
    call_unsafe_function();
    create_safe_abstract_over_unsafe_code();
    call_extern_c_code();
    unsafe_modify_static();
}


/// *const T and *mut T
/// Differences between reference and raw pointer:
/// * Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
/// * Aren’t guaranteed to point to valid memory
/// * Are allowed to be null
/// * Don’t implement any automatic cleanup
/// By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.
///
/// Why raw pointer?
///  * Interface with C together
/// *
fn dereference_raw_pointer() {
    let mut num = 2;

    //immutable raw pointer
    let r1 = &num as *const i32;
    //mutable raw pointer
    let r2 = &mut num as *mut i32;

    //raw pointer can be pointed to an unknown memory
    let address = 0x012345usize;
    let r = address as *const i32;

    //need to be unsafe block
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
}

/// call unsafe function
fn call_unsafe_function() {
    unsafe {
        unsafe_fn();
    }
}

unsafe fn unsafe_fn() {

}

/// Creating a Safe Abstraction over Unsafe Code
///
fn create_safe_abstract_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("{:?}", a);
    println!("{:?}", b);
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // let len = slice.len();
    //
    // assert!(mid <= len);
    //
    // //Error : "second mutable borrow occurs here", can't pass rust check-
    // (&mut slice[..mid], &mut slice[mid..])

    let len = slice.len();
    let ptr  = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}



///Using extern Functions to Call External Code
/// FFI: foreign function interface
/// Within the extern "C" block, we list the names and signatures of external functions
/// from another language we want to call.
/// The "C" part defines which application binary interface (ABI) the external function uses:
/// the ABI defines how to call the function at the assembly level.
/// The "C" ABI is the most common and follows the C programming language’s ABI.
///
fn call_extern_c_code() {
    unsafe {
        println!("Absolute value of call to C: {}", abs(-3));
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}


/// Calling Rust Functions from Other Languages
/// We can also use extern to create an interface that allows other languages to call Rust functions.
/// Instead of an extern block, we add the extern keyword and specify
/// the ABI to use just before the fn keyword. We also need to add a #[no_mangle]
/// annotation to tell the Rust compiler not to mangle the name of this function.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


/// Constants and immutable static variables might seem similar,
/// but a subtle difference is that values in a static variable
/// have a fixed address in memory. Using the value will always access the same data.
/// Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
///
/// Another difference between constants and static variables is that static
/// variables can be mutable. Accessing and modifying mutable static variables is unsafe.
/// Listing 19-10 shows how to declare, access, and modify a mutable static variable named COUNTER.
///
///
static mut COUNTER: i32 = 10;

/// unsafe block to Access or Modify a Mutable Static Variable
///
fn unsafe_modify_static() {
    unsafe {
        COUNTER += 1;
    }
    unsafe {
        println!("name is: {}", COUNTER);
    }
}

/// Unsafe trait
unsafe trait Foo {
}

unsafe impl Foo for i32 {
}

/// When to Use Unsafe Code
/// Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong
/// or even frowned upon. But it is trickier to get unsafe code correct because
/// the compiler can’t help uphold memory safety. When you have a reason to use unsafe code,
/// you can do so, and having the explicit unsafe annotation makes it easier to
/// track down the source of problems when they occur.

