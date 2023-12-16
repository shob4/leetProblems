use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let haystack = &args[1];
    let needle = &args[2];
    let index: i32 = -1;
    if haystack.contains(&needle) {
        let result = haystack.find(&needle);
        index = match result {
            Some(x) => x as i32,
            None => panic!()
        };
    }
    index
}
