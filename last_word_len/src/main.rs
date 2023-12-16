fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut sentence: String = String::new();
    for string in args {
        sentence.push_str(&string);
        sentence.push_str(" ");
    }

    let strings = sentence.split(" ");
    let collection = strings.collect::<Vec<&str>>();
    for i in 0..collection.len() {
        println!("{0}, {1}", collection[i], i);
    }
    let result = collection.last();
    match result {
        Some(x) => println!("{}", x),
        None => panic!("nothing there"),
    };
}
