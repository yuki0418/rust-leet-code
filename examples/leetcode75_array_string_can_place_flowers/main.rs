fn main() {
    let flowerbed: Vec<i32> = vec![1, 0, 0, 1,0,1];
    let n = 1;
    println!("output: {:?}", can_place_flowers(flowerbed, n));
}

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut count = n;

    for i in 0..flowerbed.len() {
        if count == 0 {
            break;
        }
        if flowerbed[i].is_positive() {
            continue;
        }

        let prev = *flowerbed.get(i - 1).unwrap_or(&0);
        let next = *flowerbed.get(i + 1).unwrap_or(&0);

        if prev == 0 && next == 0 {
            flowerbed[i] = 1;
            count -= 1
        }
    }

    count <= 0
}
