fn main() {
    let mut input_vec = vec![1,2,3,4,5,6,7];
    // input_vec.reverse();
    // println!("{:?}", input_vec)
    let output_vec = reverse_vec(input_vec);
    println!("{:?}", output_vec)
}


fn reverse_vec(mut input_vec: Vec<u32>) -> Vec<u32> {
    let len = input_vec.len();
    let (mut start, mut end) = (0_usize, input_vec.len() - 1);
    let mut temp = 0_u32;
    while start < end {
        temp = input_vec[start];
        input_vec[start] = input_vec[end];
        input_vec[end] = temp;
        start += 1;
        end -= 1;
        if start > len {
            break;
        }
    }
    return input_vec;
}