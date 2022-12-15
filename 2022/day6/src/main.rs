use std::fs;

fn is_unique(slice: &str, len: usize) -> bool {
    !(1..len).any(|i| slice[i..].contains(&slice[(i-1)..i]))
}


fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("data/input.txt")?;
    let len = 4;
    for i in 0..input.len() - len {
        if is_unique(&input[i..i+len], len) {
            println!("Start maker is {} at {}", &input[i..i+len], i+len);
            break;
        }
    }
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("data/input.txt")?;
    let len = 14;
    for i in 0..input.len() - len {
        if is_unique(&input[i..i+len], len) {
            println!("Start message is {} at {}", &input[i..i+len], i+len);
            break;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
