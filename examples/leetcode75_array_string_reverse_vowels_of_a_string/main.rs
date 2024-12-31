fn main() {
    let s = "IceCreAm".to_string();
    println!("output: {:?}", reverse_vowels(s));
    // AceCreIm
}

pub fn reverse_vowels(mut s: String) -> String {
    // From the description, `s` is guaranteed to be ASCII, so this is fine
    let mut bytes = unsafe { s.as_bytes_mut() };

    let mut iter = bytes.iter_mut();
    while let (Some(left), Some(right)) = (iter.find(|c| is_vowel(c)), iter.rfind(|c| is_vowel(c)))
    {
        std::mem::swap(left, right);
    }

    s
}

fn is_vowel(c: &u8) -> bool {
    matches!(c.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u')
}
