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

    determine_monotone(arr);
}

fn determine_monotone(arr: Vec<i32>) -> bool {
    let mut i = 0;
    let mut j = 1;
    let result = loop {
        if arr[j] - arr[i] != 0 {
            break arr[j] - arr[i];
        }
        if j >= arr.len() {
            break 0;
        }
        i += 1;
        j += 1;
    };

    if result == 0 {
        return false;
    }
    
    i = 0;
    j = 1;
    if result > 0 {
        while j < arr.len() {
            if arr[j] < arr[i] {
                println!("false");
                return false;
            }
            i += 1;
            j += 1;
        }
        println!("true");
        true
    } else {
        while j < arr.len() {
            if arr[j] > arr[i] {
                println!("false");
                return false;
            }
            i += 1;
            j += 1;
        }
        println!("true");
        true
    }
}
