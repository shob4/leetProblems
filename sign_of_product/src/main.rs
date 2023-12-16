fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut array: Vec<i32> = Vec::new();
    let mut i: usize = 1;
    let count = 0;

    while i < args.len() {
        if i > 0 {
            let result = args[i].parse();
            match result {
                Ok(x) => array.push(x),
                Err(e) => println!("error parsing {0}: {e:?}", args[i]),
            }
        }
        i += 1;
    }

    i = 0;
    while i < array.len() {
        if array[i] < 0 {
            count += 1;
        } else if array[i] == 0 {
            0
        }
        i += 1;
    }

    if count % 2 == 1 {
        -1
    } else {
        1
    }

}
