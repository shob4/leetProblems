fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut arr: Vec<i32> = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let result = args[i].parse();
        match result {
            Ok(x) => arr.push(x),
            Err(e) => println!("{0} failed to parse: {e:?}", args[i]),
        }
        i += 1;
    }
    
    let result = determine_progression(arr);
    if result {
        println!("true");
    } else {
        println!("false");
    }
}

fn determine_progression(mut arr: Vec<i32>) -> bool {
    arr.sort();
    let diff = arr[1] - arr[0];
    let mut i = 2;
    println!("{0}", diff);
    println!("{:?}", arr);
    while i < arr.len() {
        println!("{0} - {1} = {2}", arr[i], arr[i - 1], (arr[i] - arr[i - 1]));
        if (arr[i] - arr[i - 1]) != diff {
            return false
        }
        i += 1;
    }
    true
}
