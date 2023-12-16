use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut array: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    let mut j = 0;
   
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
    let len = array.len();
    while i < len  && j < len{
        if array[i] == 0 {
            array.remove(i);
            array.push(0);
            j += 1;
            continue;
        }
        println!("{:?}", array);
        i += 1;
    }
    println!("exited loop");
}
