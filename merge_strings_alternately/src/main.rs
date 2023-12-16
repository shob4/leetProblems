impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String{
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;
        let mut result: Option<char>;// should change result to result type
        let mut string = String::new();
        loop {
            if i == 0 {
                let result = word1.chars().nth(j);
                match result {
                    None => break,
                    Some(char) => Some(string.push(char)),
                };
                j += 1;
                i += 1;
            } else { // result should be set to equal the result of the push to string, not other
                     // way around
                let result = word2.chars().nth(k);
                match result {
                    None => break,
                    Some(char) => Some(string.push(char)),
                };
                k += 1;
                i -= 1;
            }
        }
        
        if j < word2.len() {
            string.push_str(&word2[k..]);
        } else if k < word1.len() {
            string.push_str(&word1[j..]);
        }
        string
    }
}
