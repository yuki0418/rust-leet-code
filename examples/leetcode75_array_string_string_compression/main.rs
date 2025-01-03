fn main() {
    // let mut chars = vec!['a'];
    // let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let mut chars = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'a', 'a',
    ];
    println!("output: {:?}", compress(&mut chars));
}

pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut p: usize = 0;
    let mut right: usize = 1;
    let mut counter: usize = 1;

    while right != chars.len() + 1 {
        if right == chars.len() || chars[right] != chars[right - 1] {
            chars[p] = chars[right - 1];

            if counter != 1 {
                let num_str = counter.to_string();
                for el in num_str.as_bytes().iter() {
                    p += 1;
                    chars[p] = *el as char;
                }
            }
            p += 1;
            counter = 0;
        }
        right += 1;
        counter += 1;
    }

    p as i32
}
