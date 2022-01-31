1. Array: Rust has list types to represent a sequence of things. The most basic is the array, a fixed-size list of elements of the same type. By default, arrays are immutable.
  
1. Since array is fixed-size, rust allocated stack memory for array data structure. The good part is that there is [no ownership issue on stack memory](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), 
   the downside is stack memory is limited and array size is fixed value.
The default stack size in rust for each thread [is 2M](https://doc.rust-lang.org/std/thread/#stack-size): 
   Which explains why the below code failed `stack overflow` error
```rust
fn main() {
    let given_array = ["a"; 2 * 1024 * 1024];
    println!("given_array: {:?}", given_array.len());
}
```
output:
```text
thread 'main' has overflowed its stack
atal runtime error: stack overflow
```

Highlights: 
```
Heap memory: Expensive allocations and cheap moves (ownership transfers)
Stack memory: Cheap allocations and expensive moves
```

