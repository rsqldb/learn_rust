//cargo build, it uses dev profile
//cargo build --release, it ues release profile
//cargo test --

fn main() {
    println!("Hello, world!");
    doc_test(10);
}

///
/// #Example
///
/// ```
/// let arg = 5;
/// let answer = crate::do_test(arg);
///
/// assert_eq!(15, answer)
/// ```
///
///
pub fn doc_test(val: u32) -> u32 {
    println!("val is {}", val);
    val + 10
}

// how to release package to crates.io?
// create account in creates.io and get the API token
// save API token to local ~/.cargo/credentials
