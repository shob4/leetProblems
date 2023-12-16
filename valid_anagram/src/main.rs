fn main() {
    let mut word = String::new();
    let mut anagram = String::new();
    let result: Result<String, E>;
    result = std::io::stdin().read_line(&mut word);
    result = std::io::stdin().read_line(&mut anagram);
    let mut freq: Vec<i32> = vec![0; 26];
    for char in word.chars() {
        freq[char as usize - 'a' as usize] += 1;
    }
    for char in anagram.chars() {
        let idx = char as usize - 'a' as usize;
        freq[idx] -= 1;
        if freq[idx] < 0 {
            return false
        }
    }
    for value in freq.iter() {
        if value > &0 as &i32 {
            return false
        }
    }
    true
}
