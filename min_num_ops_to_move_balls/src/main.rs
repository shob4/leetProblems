fn main() {
    j
}

// how many operations it takes to move ball to current location
pub fn get_boxes(boxes: String) -> Vec<u32> {
    let result: Option<u32>;
    let box_vec: Vec<u32>;
    let mut i: i32 = 0;
    for char in boxes.chars() {
        result = char.to_digit(10);
        match result {
            Some(u32) => box_vec.push(u32),
            None => {
                println!("{} is not a digit", char);
                break;
            },
        }
    }
    box_vec
}

pub fn get_moves_per_box(boxes: Vec<i32>) -> Vec<i32> {
    j
}
