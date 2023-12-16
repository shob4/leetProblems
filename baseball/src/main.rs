fn main() {
// int x means record new score x
// '+' means record new score that is sum of previous scores
// 'D' record new score that is double previous
// 'C' remove previous score
    let args: Vec<String> = std::env::args().collect();
    let mut score: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for i in 1..args.len() {
        let result = args[i].chars().nth(0);
        let possible_char = match result {
            Some(x) => x,
            None => continue,
        };
        let result = possible_char.is_numeric();
        if result {
            let addition = args[i].parse();
            match addition {
                Ok(x) => score.push(x),
                Err(e) => println!("how did we get here? {}", e),
            };
        } else {
            let character = possible_char;
            score = parse_instruction(character, score.clone());
        }
    }
    for i in 0..score.len() {
        sum += score[i];
    }
    println!("{0}", sum);
}

fn parse_instruction(instruction: char, mut score: Vec<i32>) -> Vec<i32> {
    match instruction {
        '+' => score.push(score[score.len() - 1] + score[score.len() - 2]),
        'D' => score.push(score[score.len() - 1] * 2),
        'C' => _ = score.remove(score.len() - 1),
        _ => panic!("an invalid instruction was given"),
    };
    println!("{:?}", score);
    score
}
