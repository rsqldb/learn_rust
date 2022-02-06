// book: https://doc.rust-lang.org/book/title-page.html

fn main() {
    let given_array = [1,2,3,4,5,6,7,8,9,10,11];
    let output_array = reverse_array(given_array);
    println!("given_array: {:?}", given_array);
    println!("output_array: {:?}", output_array);
    // array_on_stack();

    let str = "哦aa";
    println!("{}", &str[0..3]);
    println!("{}", str.chars().take(1).last().unwrap());
}

fn array_on_stack() {
    //https://doc.rust-lang.org/std/thread/#stack-size
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    let given_array = ["a"; 2 * 1024 * 1024];
    println!("given_array: {:?}", given_array.len());
}

fn reverse_array(mut input_array: [i32; 11]) -> [i32; 11] {
    let len = input_array.len();
    let mut index = 0;
    //If here input_array is a vec, this way wouldn't work, because there will be ownership issue
    for item in input_array {
        input_array[index] = input_array[len - index - 1];
        input_array[len - index - 1] = item;
        index = index + 1;
        if index > len - index -1 {
            break;
        }
    }
    return input_array;
}