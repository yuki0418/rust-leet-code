fn main() {
    // let nums = vec![1, 2, 3, 4, 5];
    // let nums = vec![5, 4, 3, 2, 1];
    // let nums = vec![2, 1, 5, 0, 4, 6];
    let nums = vec![20, 100, 10, 12, 5, 13];
    println!("output: {:?}", increasing_triplet(nums));
}

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let mut min = i32::MAX;
    let mut mid = i32::MAX;

    for i in nums.iter() {
        if min >= *i {
            min = *i;
        } else if mid >= *i {
            mid = *i
        } else {
            return true;
        }
    }

    false
}
