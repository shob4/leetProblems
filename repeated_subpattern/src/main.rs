fn main() {
    let mut s: String = String::new();
    let _result = std::io::stdin().read_line(&mut s);
    if s.len() == 1 {
        println!("false");
        return;
    }

    let mut num_div = 1;
    let mut substring: String = String::new();
    let mut check_string: String = String::new();
    let mut i = 1;
    while num_div < s.len() {
        num_div += 1;
        println!("{}", num_div);
        if s.len() % num_div == 0 {
            substring = s[0..num_div].to_string();
            let num_times = s.len() / num_div;
            while i < num_times {
                check_string.push_str(&substring);
                i += 1;
            }
            println!("substring: {0}\ncheck_string: {1}\ns: {2}", substring, check_string, s);
            if check_string == s.trim() {
                println!("true");
                return;
            }
            break;
        }
    }
    println!("false");
}
