fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqr".to_string();
    println!("output: {}", merge_alternately(word1, word2));
}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut i = 0;
    let mut flag = false;
    let mut output = "".to_string();

    while !flag {
        let a: Option<char> = word1.chars().nth(i);
        let b: Option<char> = word2.chars().nth(i);

        if let Some(c) = a {
            output += &c.to_string();
        }

        if let Some(c) = b {
            output += &c.to_string();
        }

        if i >= word1.len() && i >= word2.len() {
            flag = true
        } else {
            i += 1
        }
    }

    output.trim().to_string()
}
