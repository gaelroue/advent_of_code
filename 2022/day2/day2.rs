use std::fs;

fn shifumi(me: i32, opponent : i32) -> i32 {
    if me == opponent { return 3; }
    else if (me - opponent).rem_euclid(3) == 1 { return 6;}
    else { return 0;}
}

 

fn get_shape(strat: i32, opponent : i32) -> i32 {
    if strat == 0 { return (opponent - 1).rem_euclid(3);}
    else if strat == 1 { return opponent;}
    else { return (opponent + 1).rem_euclid(3);}
}

fn part_one() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result : i32 = input.lines().map(|line| {
        let moves = line.as_bytes();
        let left = (moves[0] - b'A') as i32;
        let right = (moves[2] - b'X') as i32;
        let ret = shifumi(right, left);
        ret + right + 1
    }).sum();

    println!("Following the first strategy, the total score would be {}", result);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result : i32 = input.lines().map(|line| {
	let moves = line.as_bytes();
	let left = (moves[0] - b'A') as i32;
	let right = (moves[2] - b'X') as i32;
	let my_move = get_shape(right, left);
	let ret = shifumi(my_move, left);
	ret + my_move + 1
    }).sum();

    println!("Following the Elf's instructions for the second column, the total score would be {}", result);
    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
