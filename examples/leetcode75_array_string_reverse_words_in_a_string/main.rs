fn main() {
    // let s = "the sky is blue".to_string();
    // let s = "  hello world  ".to_string();
    let s = "a good   example".to_string();
    println!("output: {:?}", reverse_words(s));
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .rev()
        .enumerate()
        .fold(String::with_capacity(s.len()), |acc, (i, item)| {
            if i == 0 {
                acc + item
            } else {
                acc + " " + item
            }
        })
}
