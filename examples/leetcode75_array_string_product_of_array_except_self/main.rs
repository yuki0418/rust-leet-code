fn main() {
    let nums = vec![-1, 1, 0, -3, 3];
    println!("output: {:?}", product_except_self(nums));
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut output = vec![1; nums.len()];
    let mut left = 1;

    for i in 0..output.len() {
        println!("i: {:?}, output: {:?}, left: {:?}", i, output, left);
        output[i] *= left;
        left *= nums[i];
    }

    let mut right = 1;
    for i in (0..output.len()).rev() {
        println!("i: {:?}, output: {:?}, right: {:?}", i, output, right);
        output[i] *= right;
        right *= nums[i]
    }

    output
}
