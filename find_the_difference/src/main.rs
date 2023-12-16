fn main() {
    let mut s = String::new();
    let mut t = String::new();
    let result: Result<String, E>;
    result = std::io::stdin().read_line(&mut s);
    result = std::io::stdin().read_line(&mut t);
    find_the_difference(s, t);
}

pub fn find_the_difference(s: String, t: String) -> char {
    let mut freq: Vec<i16> = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }
    for ch in t.chars() {
        let idx = ch as usize - 'a' as usize;
        freq[idx] -= 1;
        if freq[idx] < 0 {
            return ch
        }
    }
    unreachable!()
}
