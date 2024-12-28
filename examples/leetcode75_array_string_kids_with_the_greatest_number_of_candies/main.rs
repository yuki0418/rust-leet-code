fn main() {
    let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    println!("output: {:?}", kids_with_candies(candies, extra_candies));
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.clone().iter().max().unwrap().to_owned();
    let mut output: Vec<bool> = vec![];

    for candy in candies {
        let total = candy + extra_candies;
        if max_candies <= total {
            output.push(true);
        } else {
            output.push(false);
        }
    }

    output
}
