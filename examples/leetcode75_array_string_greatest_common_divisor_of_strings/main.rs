fn main() {
    let word1 = "ABCABCABC".to_string();
    let word2 = "ABCABC".to_string();
    // let word1 = "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string();
    // let word2 = "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string();
    println!("output: {}", gcd_of_strings(word1, word2));
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    println!("{:?} : {:?}", str1.clone() + &str2, str2.clone() + &str1);
    if (str1.clone() + &str2) != (str2.clone() + &str1) {
        return "".to_string();
    }
    let length = gcd(str1.len(), str2.len());
    str1[0..length].to_owned()
}

pub fn gcd(n1: usize, n2: usize) -> usize {
    println!("n1: {:?}, n2: {:?}", n1, n2);
    if n2 == 0 {
        return n1;
    }
    gcd(n2, n1 % n2)
}
