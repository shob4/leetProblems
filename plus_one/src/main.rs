fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut array: Vec<i32> = Vec::new();
    let mut i: usize = 1;
    let mut j: i32;

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
    
    i = array.len() - 1;
    j = array.len() as i32;
    while j > 0 {
        array[i] += 1; 
        if array[i] >= 10 {
            let new = array[i] % 10;
            array[i] = new;
            i -= 1;
            j -= 1;
        }
        else {
            break;
        }
    }
    if array[0] == 0 {
        array.insert(0, 1);
    }
    println!("{:?}", array);
}
